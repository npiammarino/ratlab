SRC_DIRS:= . ./src
BUILD_DIRS:= ./build ./build/lib

TESTS_UNIT:= test-linear

main: ./src/main.rs liblinear.rlib
	rustc -L ./build/lib ./src/main.rs --out-dir ./build

liblinear.rlib: directories ./src/linear.rs
	rustc --crate-type lib ./src/linear.rs --out-dir ./build/lib

directories:
	@mkdir -p $(BUILD_DIRS)

run: main
	./build/main

test-unit: $(TESTS_UNIT)
	for i in $(TESTS_UNIT); do make $i; done


test-linear: ./test/unit/test-linear.rs liblinear.rlib
	@rustc -L ./build/lib --crate-type lib ./test/unit/test-linear.rs --test;
	./test-linear
	rm test-linear
	@echo


clean:
	rm -r ./build
	find -name "*~" -type f -delete
