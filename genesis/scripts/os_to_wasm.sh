mkdir -p res
cd oracle_scripts/

for f in *; do
    if [ -d "$f" ]; then
        RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release --package $f
        cp ../target/wasm32-unknown-unknown/release/$f.wasm ../res
    fi
done
