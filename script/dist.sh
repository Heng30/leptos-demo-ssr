#! /bin/bash

rm -rf ../dist
cp -rf ../target/site ../dist
cp -rf ../target/release/leptos-demo-ssr ../dist
cp -rf ./run.sh ../dist
