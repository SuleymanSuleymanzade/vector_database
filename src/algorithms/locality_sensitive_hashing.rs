use std::collections::HashMap;
use ndarray::{Array1, Array2, arr1};
use rand::prelude::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct HashGenerator {
    projections: Array1<f64>,
    hash_map: HashMap<String, Array1<f64>>
}

impl HashGenerator {
    pub fn new(num_projections: usize) -> Self {
        let mut rng = thread_rng();
        let projections = Array1::from_shape_fn(
            num_projections, |_| rng.gen_range(-1.0..1.0));
            
        HashGenerator { 
            projections:projections,
            hash_map: HashMap::new()
        }
    }

    pub fn generate_hash(&self, inp_vector: &Array1<f64>) -> String {
        let dot_product = inp_vector.dot(&self.projections);
    
        let mut bools = String::new();
        if dot_product > 0.0 {
            bools.push('1');
        } else {
            bools.push('0');
        }
        bools
    }
    
}