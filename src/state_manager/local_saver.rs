use crate::algorithms::fast_search::FastSearch;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
//use serde_json::Result;
use serde_json::Error as SerdeError;

pub struct LocalSaver;

impl LocalSaver {
    pub fn save_state<T>(fast_search: &T, file_path: &str) -> Result<(), io::Error>
    where
        T: FastSearch + serde::Serialize,
    {
        let serialized = serde_json::to_string(fast_search)?;
        let mut file = File::create(file_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn load_state<T>(fast_search: &mut T, file_path: &str) -> Result<(), io::Error>
    where
        T: FastSearch + serde::de::DeserializeOwned,
    {
        let mut file = OpenOptions::new().read(true).open(file_path)?;
        let mut serialized = String::new();
        file.read_to_string(&mut serialized)?;
        *fast_search = serde_json::from_str(&serialized)?;
        Ok(())
    }
}