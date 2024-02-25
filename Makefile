SRC_DIRS:= . ./src
BUILD_DIRS:= ./build ./build/lib

main: ./src/main.rs liblinear.rlib
	rustc -L ./build/lib ./src/main.rs --out-dir ./build

liblinear.rlib: directories ./src/linear.rs
	rustc --crate-type lib ./src/linear.rs --out-dir ./build/lib

directories:
	mkdir -p $(BUILD_DIRS)

run: main
	./build/main

clean:
	rm -r ./build
	find -name "*~" -type f -delete
