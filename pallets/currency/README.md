## License: Unlicense


## Testing

```
cargo test -p pallet-currency --all-features

```

## Running Benchmarks
You can get a list of the available benchmarks by running:

```
./target/release/phuquocdog-node benchmark --chain dev --pallet "*" --extrinsic "*" --repeat 0
```

Then you can run a benchmark like so:

```
./target/release/phuquocdog-node benchmark --chain dev --execution=wasm --wasm-execution=compiled --pallet pallet_currency  --extrinsic=* --steps 50 --repeat 20 --output pallets/currency/src/weights.rs 

```