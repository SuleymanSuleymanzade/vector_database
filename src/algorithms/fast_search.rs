use crate::vector_db;

pub trait FastSearch{ 
    //fast search interface for LSH, KDTree and other algorithms
    //fn check_consistency(&self); // using for state checking     
    fn nearest_neighbor(&self, query:&vector_db::Vector) -> Option<&vector_db::Vector>;

}