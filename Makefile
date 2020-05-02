all:
	make build
	make www

build:
	rm -rf pkg
	wasm-pack build 

www:
	npm init wasm-app www


