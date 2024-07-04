use std::collections::HashMap;
use ndarray::{Array1, Array2};
use rand::prelude::*;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeStruct;
use serde::de::{Visitor, SeqAccess};
use serde_json;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HashGenerator {
    hash_size: usize,
    dimention_size: usize,
    projections: Array2<f64>,
    hash_map: HashMap<String, Array1<f64>>,
}

impl Serialize for HashGenerator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("HashGenerator", 4)?;
        state.serialize_field("hash_size", &self.hash_size)?;
        state.serialize_field("dimention_size", &self.dimention_size)?;
        state.serialize_field("projections", &self.projections.as_slice().unwrap())?;
        let hash_map_serialized: HashMap<String, Vec<f64>> = self.hash_map.iter()
            .map(|(k, v)| (k.clone(), v.to_vec()))
            .collect();
        state.serialize_field("hash_map", &hash_map_serialized)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for HashGenerator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct HashGeneratorHelper {
            hash_size: usize,
            dimention_size: usize,
            projections: Vec<f64>,
            hash_map: HashMap<String, Vec<f64>>,
        }

        let helper = HashGeneratorHelper::deserialize(deserializer)?;
        let projections = Array2::from_shape_vec((helper.hash_size, helper.dimention_size), helper.projections)
            .map_err(serde::de::Error::custom)?;

        let hash_map: HashMap<String, Array1<f64>> = helper.hash_map.into_iter()
            .map(|(k, v)| (k, Array1::from(v)))
            .collect();

        Ok(HashGenerator {
            hash_size: helper.hash_size,
            dimention_size: helper.dimention_size,
            projections,
            hash_map,
        })
    }
}

impl HashGenerator {
    pub fn new(hash_size: usize, dimention_size: usize) -> Self {
        let mut rng = thread_rng();
        let projections = Array2::from_shape_fn(
            (hash_size, dimention_size), |_| rng.gen_range(-1.0..1.0));
            
        HashGenerator { 
            hash_size,
            dimention_size,
            projections,
            hash_map: HashMap::new(),
        }
    }

    pub fn generate_hash(&self, inp_vector: &Array1<f64>) -> String {
        let dot_product = inp_vector.dot(&self.projections.t());
        dot_product.iter().map(|&x| if x > 0.0 { '1' } else { '0' }).collect()
    }

    pub fn insert(&mut self, inp_vector: &Array1<f64>) {
        let hash_value = self.generate_hash(inp_vector);
        self.hash_map.insert(hash_value, inp_vector.clone());
    }

    pub fn get(&self, inp_vector: &Array1<f64>) -> Option<Array1<f64>> {
        let hash_value = self.generate_hash(inp_vector);
        self.hash_map.get(&hash_value).cloned()
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LSH{
    num_tables: usize,
    hash_size: usize,
    dimention_size:usize,
    generators: Vec<Box<HashGenerator>> //expensive structure
} // MUST implement FastSearch trait (interface)

impl LSH{
    pub fn new(num_tables:usize, hash_size:usize, dimention_size:usize) -> Self{
        // Constructor
        let mut buff_generators = vec![];
        for i in 0..num_tables{
            let hash_generator = HashGenerator::new(
                hash_size,
                dimention_size
            );
            buff_generators.push(Box::new(hash_generator))
        }

        Self{
            num_tables,
            hash_size,
            dimention_size,
            generators: buff_generators
        }       
    }

    pub fn insert(&mut self, inp_vector: &Array1<f64>){
        for item in self.generators.iter_mut(){
            item.insert(inp_vector);
        }
    }

    pub fn nearest(&self, inp_vector: &Array1<f64>) -> Vec<Array1<f64>>{
        let mut buff: Vec<Array1<f64>> = vec![];
        for item in self.generators.iter(){
            let candidate = item.get(inp_vector);
            if let Some(data) = candidate{
                buff.push(data);
            }
        }
        return buff
    }
}

mod tests{
    #[test]
    fn check_lsg(){
        use super::*;
        use ndarray::{Array1, Array2, arr1};
        use ndarray;
        let mut lsh = LSH::new(5, 6, 5);
        // Create some sample input vectors
        let vector1 = arr1(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        let vector2 = arr1(&[2.0, 3.0, 4.0, 5.0, 6.0]);
        let vector3 = arr1(&[3.0, 4.0, 5.0, 6.0, 7.0]);    

        lsh.insert(&vector1);
        lsh.insert(&vector2);
        lsh.insert(&vector3);

        let vector4 = arr1(&[3.5, 4.0, 5.0, 6.0, 7.0]);    
        let res = lsh.nearest(&vector4);
        assert_eq!(res.get(0).unwrap(), vector3);
    }
}