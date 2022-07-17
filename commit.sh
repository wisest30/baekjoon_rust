#!bin/bash

if [ -z "$1" ]
  then
    echo "No argument supplied"
else
    cp ./src/main.rs ./src/solved/$1.rs
    git add ./src/solved/$1.rs
    git commit -m "solve $1"
fi