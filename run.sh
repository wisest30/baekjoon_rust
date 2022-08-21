#!bin/bash
ulimit -s unlimited

rustc -C 'link-args=-Wl,-stack_size,0x20000000' ./src/main.rs
time ./main < input.txt > output.txt

#time cargo run < input.txt > output.txt
