mod saver;
use crate::algorithms::fast_search::FastSearch;
use saver::Saver;
use std::fs;
use serde_json;

pub struct LocalSaver{}

impl Saver for LocalSaver{
    fn save_state(
        &self, 
        fast_search_obj: fast_search::FastSearch)->serde_json::Result<()>{
            let curr_state = fast_search_obj.get_state();
    }
    fn load_state(&self, path: &str) -> serde_json::Result<&FastSearch, String>;
    fn remove_state(&self, path: &str);
}



