mod vector_db;

fn main() {
    let mut db = crate::vector_db::VectorDB::new();
    db.add_vector([1.0, 2.0, 3.0]);
    db.add_vector([4.0, 5.0, 6.0]);

    // Retrieving and printing a vector
    if let Some(vector) = db.get_vector(0) {
        println!("Vector at index 0: {:?}", vector);
    }
    // Finding and printing the closest vector
    if let Some(closest) = db.find_closest([2.0, 3.0, 4.0]) {
        println!("Closest vector: {:?}", closest);
    }
}