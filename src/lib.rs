//use pyo3::ffi::_PyInterpreterState_GetEvalFrameFunc;
use pyo3::methods;

use numpy::{PyArray1, ToPyArray};

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use std::collections::HashMap;
mod algorithms;
use algorithms::vector_db as vector_db;
use algorithms::kdtree as kdtree;
use crate::algorithms::fast_search::FastSearch; 

use ndarray::{Array1, Array2};

use algorithms::locality_sensitive_hashing::HashGenerator;
use algorithms::locality_sensitive_hashing::LSH;

#[pyclass]
struct VectorDB{
    lsh: LSH
}

#[pymethods]
impl VectorDB{
    #[new] //constructor
    fn new(
        num_tables:usize,
        hash_size: usize,
        dimention_size:usize
    ) -> VectorDB{
        VectorDB{
            lsh: LSH::new(
                num_tables,
                hash_size,
                dimention_size
            )
        }
    }

    fn insert(&mut self, py: Python, inp_vector: &PyArray1<f64>) {
        let vector = inp_vector.to_owned_array();
        self.lsh.insert(&vector);
    }

    fn nearest(&self, py: Python, inp_vector: &PyArray1<f64>) -> Vec<Py<PyArray1<f64>>> {
        let vector = inp_vector.to_owned_array();
        let nearest = self.lsh.nearest(&vector);
        nearest.into_iter().map(|v| v.to_pyarray(py).to_owned()).collect()
    }
}

#[pymodule]
fn vectordb(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<VectorDB>()?;
    Ok(())
}