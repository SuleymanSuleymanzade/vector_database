use crate::vector_db;
use serde_json;
pub trait FastSearch{ 
    //fast search interface for LSH, KDTree and other algorithms
    //fn check_consistency(&self); // using for state checking     
    fn nearest_neighbor(&self, query:&vector_db::Vector) -> Option<&vector_db::Vector>;
    fn get_state(&self) -> serde_json::Result<String>;
}