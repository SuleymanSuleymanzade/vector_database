<center>

![logo](./assets/vector_db.png)

### Vector Database Project. </center>


![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Git](https://img.shields.io/badge/git-%23F05033.svg?style=for-the-badge&logo=git&logoColor=white) ![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white) ![AWS](https://img.shields.io/badge/AWS-%23FF9900.svg?style=for-the-badge&logo=amazon-aws&logoColor=white) ![NumPy](https://img.shields.io/badge/numpy-%23013243.svg?style=for-the-badge&logo=numpy&logoColor=white) ![PyTorch](https://img.shields.io/badge/PyTorch-%23EE4C2C.svg?style=for-the-badge&logo=PyTorch&logoColor=white) ![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)

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
pyo3 = { version = "0.18", features = ["extension-module"] }
ndarray = "0.15"
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_derive = "1.0"
serde_json = "1.0"

[lib]
crate-type = ["cdylib"]
```

Architecture:

Currently the project archiecture includes the structs responsible for the algorithms and storage.

![logo](./assets/architecture_vec_db.svg)

---

for contribution, open the [link](contribution.md).


The project currently under the development.