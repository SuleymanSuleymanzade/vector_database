use std::collections::HashMap;

mod algorithms;
mod state_manager;

use state_manager::local_saver::LocalSaver;
use algorithms::vector_db as vector_db;
use algorithms::kdtree as kdtree;
use crate::algorithms::fast_search::FastSearch; 

fn main() {
    // let mut db = crate::vector_db::VectorDB::new();
    // db.add_vector([1.0, 2.0, 3.0]);
    // db.add_vector([4.0, 5.0, 6.0]);

    // // Retrieving and printing a vector
    // if let Some(vector) = db.get_vector(0) {
    //     println!("Vector at index 0: {:?}", vector);
    // }
    // // Finding and printing the closest vector
    // if let Some(closest) = db.find_closest([2.0, 3.0, 4.0]) {
    //     println!("Closest vector: {:?}", closest);
    // }

    let points = vec![
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
        [2.0, 3.0, 4.0]];
        
    //let kd_tree_node = kdtree::KDTree::build(points, 0);
    //let kd_tree_object: kdtree::KDTree = kdtree::KDTree::new(kd_tree_node);
    
    
    let state = HashMap::from([("depth".to_string(), 2)]); // Using state provides universal trait interface for implementations
    let mut kd_tree_obj = kdtree::KDTree::new(points, state);
    
    if let Some(nearest) = kd_tree_obj.nearest_neighbor(&[3.0, 3.0, 3.0]) {
            println!("Nearest neighbor: {:?}", nearest);
    }

    kd_tree_obj.insert([3.0, 3.1, 2.9]);


    if let Some(nearest) = kd_tree_obj.nearest_neighbor(&[3.0, 3.0, 3.0]) {
        println!("Nearest neighbor: {:?}", nearest);
    }

    println!("{:?}", kd_tree_obj); // check for state

    println!("-------------------------------------------");

    let state = kd_tree_obj.get_state().unwrap();
    //let mut rr:String = state.unwrap();
    //rr.push_str("hello");
    //println!("{}", rr);

    LocalSaver::save_state(
        &kd_tree_obj, 
        "local_file.json").expect("failed to save state");
    //ls local_saver = LocalSaver();
    
}
