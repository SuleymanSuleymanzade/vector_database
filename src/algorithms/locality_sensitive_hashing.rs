use std::collections::HashMap;
use ndarray::{Array1, Array2, arr1};
use rand::prelude::*;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeStruct;
use serde::de::{Visitor, SeqAccess};
use serde_json;

#[derive(Debug)]
#[allow(dead_code)]
pub struct HashGenerator {
    projections: Array2<f64>,
    hash_map: HashMap<String, Vec<String>>,
}

impl Serialize for HashGenerator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("HashGenerator", 2)?;
        state.serialize_field("projections", &self.projections.as_slice().unwrap())?;
        state.serialize_field("hash_map", &self.hash_map)?;
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
            projections: Vec<f64>,
            hash_map: HashMap<String, Vec<String>>,
        }

        let helper = HashGeneratorHelper::deserialize(deserializer)?;
        let projections = Array2::from_shape_vec((helper.projections.len() / 2, 2), helper.projections)
            .map_err(serde::de::Error::custom)?;
        
        Ok(HashGenerator {
            projections,
            hash_map: helper.hash_map,
        })
    }
}


impl HashGenerator {
    pub fn new(hash_size: usize, inp_dimensions: usize) -> Self {
        let mut rng = thread_rng();
        let projections = Array2::from_shape_fn(
            (hash_size, inp_dimensions), |_| rng.gen_range(-1.0..1.0));
            
        HashGenerator { 
            projections: projections,
            hash_map: HashMap::new(),
        }
    }

    pub fn generate_hash(&self, inp_vector: &Array1<f64>) -> String {
        let dot_product = inp_vector.dot(&self.projections.t());
        dot_product.iter().map(|&x| if x > 0.0 { '1' } else { '0' }).collect()
    }

    pub fn insert(&mut self, inp_vector: &Array1<f64>, label: String) {
        let hash_value = self.generate_hash(inp_vector);
        self.hash_map.entry(hash_value).or_insert_with(Vec::new).push(label);
    }

    pub fn get(&self, inp_vector: &Array1<f64>) -> Vec<String> {
        let hash_value = self.generate_hash(inp_vector);
        self.hash_map.get(&hash_value).cloned().unwrap_or_else(Vec::new)
    }
}