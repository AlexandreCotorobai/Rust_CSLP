# Rust Tutorial 

### CSLP - Group 7
- Alexandre
- Joaquim
- Bernardo

## Requirements
Rust Installation: https://www.rust-lang.org/tools/install

## Getting Started

### Installing rustup on Linux or macOS
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Compiling and running a Rust program
To create a project we will be using a Rust’s build system and package manager named Cargo.

Check if cargo is installed:
```bash
$ cargo --version
```
If so you can create a project with this simple command:
```bash
$ cargo new hello_cargo
```
Cargo has generated a “Hello, world!” program for you. The project Cargo generated placed the code in the src directory and we have a Cargo.toml configuration file in the top directory.

Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. There’s a place for everything, and everything is in its place.

```bash
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```
This command creates an executable file in target/debug/hello_cargo (or target\debug\hello_cargo.exe on Windows) rather than in your current directory. 
You can run the created executable using ``./target/debug/hello_cargo``.

We can also use cargo run to compile the code and then run the resultant executable all in one command:
```bash
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

``cargo check`` is also a powerfull command that quickly checks your code to make sure it compiles but doesn’t produce an executable (good for checking if your work still compiles).
