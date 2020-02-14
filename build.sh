#!/bin/bash

# build frontend
mkdir -p ./build/static
cd ./zorius/
ng build --aot --prod --output-path=./../build/static/

# now build backend
cd ../
cargo build --release

# copy backend release files 
cp ./target/release/zorius ./build/

