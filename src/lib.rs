use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use std::collections::HashMap;
mod algorithms;
use algorithms::vector_db as vector_db;
use algorithms::kdtree as kdtree;
use crate::algorithms::fast_search::FastSearch; 


#[pyclass]
struct VectorDBInterface{
    
}

// #[pymethods]
// impl VectorDBInterface{
//     #[new] //constructor
//     fn new(points: Vec<)
// }