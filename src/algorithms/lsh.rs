use std::collections::HashMap;

pub struct LSHNode{
    hash_table: HashMap<String, Vec<f32>>
}


impl LSHNode{
    pub fn get_vector(&self, key:&str) -> Option<Vec<f32>>{
        self.hash_table.get(key)
    }
}