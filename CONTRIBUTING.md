# 🌍 Contributing.

Contributions are welcome, and they are greatly appreciated! Every little bit helps, and credit will always be given.

## Getting Started!

Ready to contribute? Here's how to set up `vector_database` for local development.

1. Fork the `vector_database` repo on GitHub.
2. Clone your fork locally:

	```sh
	git clone git@github.com:your_name_here/vector_database.git
	```

1. Navigate to the recently created directory:

	```sh
	cd  vector_database 
	```
1. Install the main dependencies:

	```toml
    [dependencies]
    pyo3 = { version = "0.18", features = ["extension-module"] }
    ndarray = "0.15"
    rand = "0.8"
    serde = { version = "1.0", features = ["derive"] }
    serde_derive = "1.0"
    serde_json = "1.0"

	```
    
1. Create a branch for local development:

	```sh
	git checkout -b name-of-your-bugfix-or-feature
	```

Now you can make your changes locally.

1. Commit your changes and push your branch to GitHub:

	```sh
	git add .
	git commit -m "Your detailed description of your changes."
	git push origin name-of-your-bugfix-or-feature
	```

1. Submit a pull request through the GitHub website.

Thank you for helping us improve!