use crate::algorithms::fast_search;

trait Saver{
    fn save_state(&self, fast_search_obj: fast_search::FastSearch);
    fn load_state(&self, path: &str) -> &FastSearch;
    fn remove_state(&self, path: &str);
}