#!/bin/bash

mkdir ./static
cd ./zorius/
ng build --aot --output-path=./../static/

cd ../

cargo run

