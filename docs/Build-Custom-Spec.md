### Build

Build customSpecRaw for testnet: 

```
./target/release/phuquocdog-node build-spec --disable-default-bootnode --chain quark > phuquocdog-json/customSpec-Testnet.json

```

```
./target/release/phuquocdog-node build-spec --chain=phuquocdog-json/customSpec-Testnet.json --raw --disable-default-bootnode > resources/octopus-testnet.json

```

Build customSpecRaw for mainnet: 


```
./target/release/phuquocdog-node build-spec --chain=phuquocdog-json/customSpec-Mainnet.json --raw --disable-default-bootnode > resources/octopus-mainnet.json

```


./target/release/phuquocdog-node benchmark --chain=dev --execution=wasm --wasm-execution=compiled --pallet=pallet_currency --extrinsic=*  --steps=20 --repeat=10 --heap-pages=4096  --raw --template=./.maintain/pallet-weight-template.hbs --output=./pallets/currency/src/weights.rs
