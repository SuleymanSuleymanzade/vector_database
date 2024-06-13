mod saver;
use crate::algorithms::fast_search::FastSearch;
use saver::Saver;
use std::fs;

pub struct LocalSaver{

}

impl Saver for LocalSaver{
    fn save_state(&self, fast_search_obj: fast_search::FastSearch){
        
    }
    fn load_state(&self, path: &str) -> &FastSearch;
    fn remove_state(&self, path: &str);
}

