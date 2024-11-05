# Vector Converter

A simple Rust library to convert between vectors of different types, such as converting a vector of integers to a vector of strings and vice versa.

## Features

- Convert a vector of integers to a vector of strings.
- Convert a vector of strings to a vector of integers (with error handling).

## Installation

To use the `vector_converter` library in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
vector_converter = "0.1"
```
## Example Usage
Convert Integer Vector to String Vector
```rust
use vector_converter::int_vec_to_string_vec;

fn main() {
    let int_vec = vec![1, 2, 3];
    let string_vec = int_vec_to_string_vec(int_vec);
    println!("{:?}", string_vec);  // Output: ["1", "2", "3"]
}
```
Convert String Vector to Integer Vector
```rust
use vector_converter::string_vec_to_int_vec;

fn main() {
    let string_vec = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    match string_vec_to_int_vec(string_vec) {
        Ok(int_vec) => println!("{:?}", int_vec),  // Output: [1, 2, 3]
        Err(e) => println!("Error: {}", e),
    }
}
```
## License
This project is licensed under the MIT License

## Author
Ben Santora <bensatlantik@gmail.com>

