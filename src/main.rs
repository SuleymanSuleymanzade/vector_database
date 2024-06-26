use std::collections::HashMap;

mod algorithms;
mod state_manager;

use algorithms::locality_sensitive_hashing::HashGenerator;
use algorithms::locality_sensitive_hashing::LSH;

use ndarray::{Array1, Array2, arr1};
use ndarray;
use state_manager::local_saver::LocalSaver;
use algorithms::vector_db as vector_db;
use algorithms::kdtree as kdtree;
use crate::algorithms::fast_search::FastSearch; 

fn main() {
    // // let mut db = crate::vector_db::VectorDB::new();
    // // db.add_vector([1.0, 2.0, 3.0]);
    // // db.add_vector([4.0, 5.0, 6.0]);

    // // // Retrieving and printing a vector
    // // if let Some(vector) = db.get_vector(0) {
    // //     println!("Vector at index 0: {:?}", vector);
    // // }
    // // // Finding and printing the closest vector
    // // if let Some(closest) = db.find_closest([2.0, 3.0, 4.0]) {
    // //     println!("Closest vector: {:?}", closest);
    // // }

    // let points = vec![
    //     [1.0, 2.0, 3.0],
    //     [4.0, 5.0, 6.0],
    //     [2.0, 3.0, 4.0]];
        
    // //let kd_tree_node = kdtree::KDTree::build(points, 0);
    // //let kd_tree_object: kdtree::KDTree = kdtree::KDTree::new(kd_tree_node);
    
    
    // let state = HashMap::from([("depth".to_string(), 2)]); // Using state provides universal trait interface for implementations
    // let mut kd_tree_obj = kdtree::KDTree::new(points, state);
    
    // if let Some(nearest) = kd_tree_obj.nearest_neighbor(&[3.0, 3.0, 3.0]) {
    //         println!("Nearest neighbor: {:?}", nearest);
    // }

    // kd_tree_obj.insert([3.0, 3.1, 2.9]);


    // if let Some(nearest) = kd_tree_obj.nearest_neighbor(&[3.0, 3.0, 3.0]) {
    //     println!("Nearest neighbor: {:?}", nearest);
    // }

    // //println!("{:?}", kd_tree_obj); // check for state

    // println!("-------------------------------------------");

    // let state = kd_tree_obj.get_state().unwrap();
    // //let mut rr:String = state.unwrap();
    // //rr.push_str("hello");
    // //println!("{}", rr);

    // // LocalSaver::save_state(
    // //     &kd_tree_obj, 
    // //     "local_file.json").expect("failed to save state");
    // // //ls local_saver = LocalSaver();
    // println!("----------------------------------");

    // LocalSaver::load_state(
    //     &mut kd_tree_obj,
    //      "local_file.json").expect("Failed to load state");

    // println!("{:?}", kd_tree_obj);


    println!("------------------------------------------");
    let mut lsh = LSH::new(5, 6, 5);
    // Create some sample input vectors
    let vector1 = arr1(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    let vector2 = arr1(&[2.0, 3.0, 4.0, 5.0, 6.0]);
    let vector3 = arr1(&[3.0, 4.0, 5.0, 6.0, 7.0]);    

    lsh.insert(&vector1);
    lsh.insert(&vector2);
    lsh.insert(&vector3);

    let vector4 = arr1(&[3.5, 4.0, 5.0, 6.0, 7.0]);    

    println!("----------------------------------------------------------------------");
    let res = lsh.nearest(&vector4);
    println!("{:?}", res);


    let arr = vec![2,5,77,3,2,5,4,3,54,9,7,10];
    let rr: Vec<i32> = arr
        .iter()
        .map(|&x| x + 2)
        .filter(|&x| x > 3)
        .collect();

    println!("{:?}", rr);

}


// fn main() {
//     // Create a new HashGenerator
//     let mut hash_gen = HashGenerator::new(10, 5);

//     // Create some sample input vectors
//     let vector1 = arr1(&[1.0, 2.0, 3.0, 4.0, 5.0]);
//     let vector2 = arr1(&[2.0, 3.0, 4.0, 5.0, 6.0]);
//     let vector3 = arr1(&[3.0, 4.0, 5.0, 6.0, 7.0]);

//     // Insert vectors with corresponding labels
//     hash_gen.insert(&vector1);
//     hash_gen.insert(&vector2);
//     hash_gen.insert(&vector3);

//     // Retrieve vectors based on input vectors
//     let retrieved_vector1 = hash_gen.get(&vector1);
//     let retrieved_vector2 = hash_gen.get(&vector2);
//     let retrieved_vector3 = hash_gen.get(&vector3);

//     // Print the results
//     println!("Retrieved vector for vector1: {:?}", retrieved_vector1);
//     println!("Retrieved vector for vector2: {:?}", retrieved_vector2);
//     println!("Retrieved vector for vector3: {:?}", retrieved_vector3);

//     // Serialize the HashGenerator to a JSON string
//     let serialized = serde_json::to_string(&hash_gen).unwrap();
//     println!("Serialized HashGenerator: {}", serialized);

//     // Deserialize the JSON string back to a HashGenerator
//     let deserialized: HashGenerator = serde_json::from_str(&serialized).unwrap();
//     println!("Deserialized HashGenerator: {:?}", deserialized);

//     // Verify that the deserialized HashGenerator works as expected
//     let retrieved_vector1_deserialized = deserialized.get(&vector1);
//     println!("Retrieved vector for vector1 from deserialized HashGenerator: {:?}", retrieved_vector1_deserialized);
// }