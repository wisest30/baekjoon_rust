#!bin/bash
ulimit -s unlimited

# for debuging at panic
#rustc -C 'link-args=-Wl,-stack_size,0x20000000' ./src/main.rs
#RUST_BACKTRACE=1 ./main < input.txt > output.txt

# debug mode
#rustc -C 'link-args=-Wl,-stack_size,0x20000000' ./src/main.rs
#time ./main < input.txt > output.txt

# optimize mode
rustc -C 'link-args=-Wl,-stack_size,0x20000000' -C 'opt-level=3' ./src/main.rs
time ./main < input.txt > output.txt

#time cargo run < input.txt > output.txt
