#!/bin/bash

all: build

run:
	cargo leptos watch

tailwind-watch:
	npm run watch

build:
	cargo leptos build --release

dist: build
	cd ./script && ./dist.sh
	npm run build

clean:
	cargo clean

