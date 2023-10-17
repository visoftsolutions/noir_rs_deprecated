cd barretenberg/aztec-packages/barretenberg/cpp/
# Clean.
rm -rf ./build
rm -rf ./build-wasm

PRESET=default
cmake --preset $PRESET -DCMAKE_BUILD_TYPE=RelWithAssert
cmake --build --preset $PRESET ${@/#/--target }
