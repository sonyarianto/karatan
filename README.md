# Karatan

Karatan is a template/sample/skeleton/boilerplate web framework implemented in Rust using Actix Web. Initially to makes my life easy. It contains simple sample to start your own backend system. Is it performant? I am not sure since I am still learning Rust as well.

## Requirements

- Rust

## How to run?

- Clone the repo and go to the project directory.
- Type `cargo build` and then `cargo run` to run the program.
- It  will run the web server on port `8000`, try at http://localhost:8000.

## Available sample routes

- (GET) / => Return string "Hello, World!"
- (GET) /about => Return string "This is about page"
- (POST) /upload => Imaginary path for POST end-point, still unimplemented
- (GET) /json => Return JSON data from static string
- (GET) /{name}/age/{age} => Return string with name and age from the URL
- (GET) /external/api/1 => Return JSON data from external API (https://jsonplaceholder.typicode.com/todos)
- (GET) /external/api/2 => Return JSON data from external hidden API with a bearer token

## License

MIT

Copyright &copy; 2024 Sony Arianto Kurniawan <<sony@sony-ak.com>> and contributors.
