#!bin/bash
cp ./src/solved/$1.rs ./src/main.rs
cargo run < input.txt > output.txt