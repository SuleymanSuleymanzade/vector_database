### Vector Database Project.

![logo](./assets/vector_db.png)


Open source project written in Rust for vector database retrieval storage system.

### Features.

- Stores vector representation of any object
- Fast search via special data structures:
    Fast Search via:
    - KDTrees (implemented)
    - Locality-Sensitive Hashing (on build)

- integration for HDFS and S3 file systems 
- support of PyTorch vectors
- support of Python/Mojo APIs
- UI based on Actix or Rocket Rust Backend

#### Dependencies:

```toml
[package]
name = "vector_database"
version = "0.1.0"
edition = "2021"
[dependencies]
```

The project is on developing stage.