mod saver;
use crate::algorithms::fast_search::FastSearch;
use saver::Saver;
use std::fs;
use serde_json;

pub struct LocalSaver{}

impl Saver for LocalSaver{
    fn save_state(&self, fast_search_obj: fast_search::FastSearch,file_path: &str)->serde_json::Result<()>{
        let curr_state:ser::Result<String> = fast_search_obj.get_state();
        // Handle the result of serialization
        match curr_state {
            Ok(state) => {
                // Open the file in write mode
                let mut file = std::fs::File::create(file_path)
                    .map_err(|e| serde_json::Error::custom(e.to_string()))?;
                // Write serializebla file
                std::io::Write::write_all(&mut file, state.as_bytes())
                    .map_err(|e| serde_json::Error::custom(e.to_string()))?;
                
                Ok(())
            },
            Err(e) => Err(e),
        }
    }
    fn load_state(&self, path: &str) -> serde_json::Result<&FastSearch, String>{
        // Read the file content
        let file_content = fs::read_to_string(path)
            .map_err(|e| serde_json::Error::custom(e.to_string()))?;

        // Deserialize the content into a concrete type that implements FastSearch
        let fast_search_obj: Box<FastSearchImpl> = serde_json::from_str(&file_content)?;
        Ok(fast_search_obj)

    }
    fn remove_state(&self, path: &str){
        let _ = fs::remove_file(path);
    }
}



