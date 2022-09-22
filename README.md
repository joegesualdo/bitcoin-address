
# Bitcoin Address
> Utilities for bitcoin addresses

---

**⚠️ This is experimental. Please use at your own risk.⚠️**

---

## Install
> Add package to Cargo.toml file
```rust
[dependencies]
bitcoin-address = "0.1.4"
```

## Usage:
```rust
use bitcoin_address::{
    is_legacy,
    is_nested_segwit,
    is_segwit_native,
    is_segwit_v0,
    is_segwit_v1,
    is_taproot,
}

let legacy_address = "1J9uwBYepTm5737RtzkSEePTevGgDGLP5S".to_string();
let nested_segwit_address = "37u4L57bLqZ8NL9bs1GNX2x52KxviDfvPp".to_string();
let native_segwit_address = "bc1qfvmj8jse4r7203mrchfyt24sjcpna3s2y35ylp".to_string();
let taproot_address =
    "bc1p8denc9m4sqe9hluasrvxkkdqgkydrk5ctxre5nkk4qwdvefn0sdsc6eqxe".to_string();

is_nested_segwit(&nested_segwit_address) // => true
is_legacy(&legacy_address) // => true
is_segwit_native(&nested_segwit_address) // => true
is_segwit_v0(&nested_segwit_address) // => true
is_segwit_v0(&native_segwit_address) // => true
is_segwit_v1(&taproot_address) // => true
is_taproot(&taproot_address) // => true
```
## Resources
To learn about the types of bitcoin addresses, see the [RESOURCES.md](./RESOURCES.md)

## Related
- [bitcoid-request](https://github.com/joegesualdo/bitcoind-request) - Request Bitcoin blockchain data from a node
- [bitcoin-node-query](https://github.com/joegesualdo/bitcoin-node-query) - Query Bitcoin Node for information
- [bitcoin-terminal-dashboard](https://github.com/joegesualdo/bitcoin-terminal-dashboard) - Bitcoin Dashboard in the terminal

## License
MIT © [Joe Gesualdo]()
 

