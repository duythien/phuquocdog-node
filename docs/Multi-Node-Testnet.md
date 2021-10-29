## Multi-Node Testnet

### Generate node key

Generate pubic key from a secret phrase. This secret phrase is dedicated to development and should not be used in any other places.

```
subkey inspect --scheme ed25519 "fire penalty pony chase gift loan grid mule tape wrestle stuff salute"

```

```
Secret phrase `fire penalty pony chase gift loan grid mule tape wrestle stuff salute` is account:
  Secret seed:       0x09c047c99b49d03c96f5915497cc5e7ffb0fce31b732abf35f2d7d1bfd89de13
  Public key (hex):  0x74a8cfbadb5d2b0178ec124791bfa8346ac3550a4f689923c806428090055277
  Public key (SS58): 5EhfZPbz9JVAXWmHpKA1zc6jpBZXS3ExvmrvZrjZ6AMa2Vzq
  Account ID:        0x74a8cfbadb5d2b0178ec124791bfa8346ac3550a4f689923c806428090055277
  SS58 Address:      5EhfZPbz9JVAXWmHpKA1zc6jpBZXS3ExvmrvZrjZ6AMa2Vzq

```

Save this public key 0x74a8cfbadb5d2b0178ec124791bfa8346ac3550a4f689923c806428090055277

### Config validators' session keys

Config validators secret phrase in prepare-test-net.sh

```
# Copy paste your mnemonic here.
SECRET="panda dose welcome ostrich brief pull lawn table arrest worth ranch faculty"
```

sh scripts/prepare-test-net.sh 2

```
(
// 5FNCTJVDxfFnmUYKHqbJHjUi7UFbZ6pzC39sL6E5RVpB4vc9
hex!["920c238572e2b31c2efd19dad1a5674c8188388d9a30d0d01847759a5dc64069"].into(),
// 5GgaLpTUcgbCTGnwVkCjSSzZ5jTaEPuxtWGRDhi8M1BP1hTs
hex!["cc4c78c7f22298f17e0e2dcefb7cff85b30e19dc1699cb9d1de00e5ea65a433d"].into(),
// 5Fm7Lc3XDxxbH4LBKxn1tf44P1R5M5cm2vmuLZbUnPFLfu5p
hex!["a3859016b0b17b7ed6a5b2efcb4ce0e2b6b56ec8594d416c0ea3685929f0a15c"].unchecked_into(),
// 5CyLUbfTe941tZDvQQ5AYPXZ6zzqwS987DTwFGnZ3yPFX5wB
hex!["2824087e4d670acc6f2ac4251736b7fb581b5bff414437b6abc88dc118ea8d5c"].unchecked_into(),
// 5CahSqUXepwzCkbC7KNUSghUcuJxPDPKiQ4ow144Gb9qBPsX
hex!["16dffa9a82c7bb62f0f9929407223bf156458a4e7970ec4007ab2da7fb389f7d"].unchecked_into(),
// 5Eeard4qtNM8DBvqDEKn5GBAspbT7QEvhAjxSsYePB26XAiJ
hex!["724f3e6ec8a61ea3dc5b76c00a049f84fd7f212443b01241e0a2bb4ce503b345"].unchecked_into(),
),
(
// 5DP3mCevjzqrYhJgPpQFkpoERKg55K422u5KiRGPQaoJEgRH
hex!["3a39a8d0654e0f52b2ee8202ed3488e7a82650dde0daadaddbc8ea825e408d13"].into(),
// 5HeTTicL5u17JCkDhAwcAHUXMGEzXbDLjPYmNC5ahKhwaLgt
hex!["f6eb0cff5244d7437ed659ac34e6ea66daa857f3d1c580f452b8512ae7fdba0f"].into(),
// 5FKFid7kAaVFkfbpShH8dzw3wJipiuGPruTzc6WB2WKMviUX
hex!["8fcd640390db86812092a0b2b244aac9d8375be2c0a3434eb9062b58643c60fb"].unchecked_into(),
// 5G4AdD8rQ6MHp2K1L7vF1E43eX69JMZDQ1vknonsALwGQMwW
hex!["b087cc20818f98e543c55989afccd3ec28c57e425dae970d9dd63cad806c1f6d"].unchecked_into(),
// 5DknzWSQVCpo7bNf2NnBsjb529K2WVpvGv6Q3kn9RgcFgoeQ
hex!["4acf560d0aa80158ee06971c0ebbf4e6a1a407e6de2df16a003a765b73e63d7b"].unchecked_into(),
// 5DhZENrJzzaJL2MwLsQsvxARhhAPCVXdHxs2oSJuJLxhUsbg
hex!["485746d4cc0f20b5581f24b30f91b34d49a7b96b85bb8ba202f354aea8e14b1f"].unchecked_into(),
),
(
// 5DJQ1NXeThmu2N5yQHZUsY64Lmgm95nnchpRWi1nSBU2rgod
hex!["36ad94b252606800bc80869baf453663ac2e9276e83f0401107384c053552f3e"].into(),
// 5EWQq4ns7miu8B8ArsspZ9KBHX6gwjJXptJq5dbLgQucZvdc
hex!["6c1386fd76e4eec0365a439db0decae0d5d715e33db934bc44be28f73df50674"].into(),
// 5EUsrdaXAAJ87Y7yCRdrYKeyHdTYbSr9tJFCYEy12CNap2v2
hex!["6ae80477725a1e4f3194fac59286662ea491c9461cb54909432228351be3474a"].unchecked_into(),
// 5FHCHVMPD9VfpzMcGVyL7gqkq2Rd9NomkHFHP8BzP8isUBnh
hex!["8e3b579b007999dce44a28bb266f73b54e6f7ec219c495ae23fe0dc3c101e158"].unchecked_into(),
// 5GRarw8oivnRh5ViPC9kH6ztbPNiyrfb61BitYz2YzhoqS4L
hex!["c0dd89e234665e119ac8396af69c37d1956ffbf4a0173c21ee5872fea2366026"].unchecked_into(),
// 5CLfsFaNYPGQvpYkroN1qrWLt54Xpmn6shAxdE45bCy1cvgv
hex!["0c2d3a4c604c4ad68e285cc1c401dd2665c1cd7193b16d4d9c854c27a9238a1a"].unchecked_into(),
),

```
Replace the session keys from vec![...] in node/src/chain_spec.rs with above generated keys.

```
fn mainnet_genesis_constuctor() -> GenesisConfig {
    let initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)> = vec![...];
    #--snip--
}

```

Recompile

```
cargo build --release --features=runtime-benchmarks

```

Generate pubic key from a secret phrase. This secret phrase is dedicated to development and should not be used in any other places.

### Config root key
Generate pubic key from a secret phrase. This secret phrase is dedicated to development and should not be used in any other places.

```
subkey inspect --scheme sr25519 "royal novel glad piece waste napkin little pioneer decline fancy train sell"

```

```
Secret phrase `royal novel glad piece waste napkin little pioneer decline fancy train sell` is account:
  Secret seed:       0x5b482eab1018eaee6293d3aaf15e4cc26fedd711b1ad4fe127adc11367ac3e9b
  Public key (hex):  0xa2bf32e50edd79c181888da41c80c67c191e9e6b29d3f2efb102ca0e2b53c558
  Public key (SS58): 5Fk6QsYKvDXxdXumGdHnNQ7V7FziREy6qn8WjDLEWF8WsbU3
  Account ID:        0xa2bf32e50edd79c181888da41c80c67c191e9e6b29d3f2efb102ca0e2b53c558
  SS58 Address:      5Fk6QsYKvDXxdXumGdHnNQ7V7FziREy6qn8WjDLEWF8WsbU3

```

Remove prefix 0x of public key, replace the root key in node/src/chain_spec.rs

```
let root_key: AccountId = hex![
    // 5Fk6QsYKvDXxdXumGdHnNQ7V7FziREy6qn8WjDLEWF8WsbU3
    "a2bf32e50edd79c181888da41c80c67c191e9e6b29d3f2efb102ca0e2b53c558"
].into();

### Launch first node

Launch first node with the public key above.

```
./target/release/phuquocdog-node --chain quark -d data/validator1 --name validator1 --in-peers 256 --validator --ws-external --rpc-cors all --rpc-methods=unsafe --node-key 0x74a8cfbadb5d2b0178ec124791bfa8346ac3550a4f689923c806428090055277


### Launch other two nodes

```
./target/release/phuquocdog-node --chain quark -d data/validator2 --name validator2 --validator --port 30334 --ws-port 9946 --rpc-port 9934 --ws-external --rpc-cors all --rpc-methods=unsafe --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWRm651Kd5GmsLTHJbgX5chQS5npx9ttLgo46UsegCMoNM
```


```
./target/release/phuquocdog-node --chain quark -d data/validator3 --name validator3 --validator --port 30335 --ws-port 9947 --rpc-port 9935 --ws-external --rpc-cors all --rpc-methods=unsafe --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWRm651Kd5GmsLTHJbgX5chQS5npx9ttLgo46UsegCMoNM

```

### Setup node session keys

- Copy validators' session keys to babe1 ~ 3, gran1 ~ 3, imol1 ~ 3, audi1 ~ 3, add prefix 0x

- Fill right secret phrase

- Run command to setup node session keys

```
cd scripts/session_keys
sh run.sh

```
Note that the format request should be like this

```
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "author_insertKey",
  "params": ["<aura/gran>", "<mnemonic phrase>", "<public key>"]
}

```