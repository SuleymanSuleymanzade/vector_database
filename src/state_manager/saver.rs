use crate::algorithms::fast_search;
use serde_json;

trait Saver{
    fn save_state(&self, fast_search_obj: fast_search::FastSearch) -> serde_json::Result<()>;
    fn load_state(&self, path: &str) -> serde_json::Result<&FastSearch, String>;
    //fn remove_state(&self, path: &str);
}