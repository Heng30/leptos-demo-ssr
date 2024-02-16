#!/bin/bash

all: build

run:
	cargo leptos watch

build:
	cargo leptos build --release

dist: build
	cd ./script && ./dist.sh

clean:
	cargo clean

