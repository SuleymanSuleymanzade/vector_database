use crate::algorithms::fast_search;

trait Saver{
    fn save_state(fast_search_obj: fast_search::FastSearch);
    fn load_state(path: &str) -> &FastSearch;
    fn remove_state(path: &str);
}