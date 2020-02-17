# typed_phy 
(TODO: maybe I should rename the lib?)

[![CI status](https://github.com/WaffleLapkin/typed_phy/workflows/Continuous%20integration/badge.svg)](https://github.com/WaffleLapkin/arraylib/actions)
[![Telegram](https://img.shields.io/badge/tg-WaffleLapkin-9cf?logo=telegram)](https://vee.gg/t/WaffleLapkin)
[![docs.rs](https://img.shields.io/badge/docs.rs-typed_phy-blue.svg)](https://docs.rs/typed_phy)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io/crates/typed_phy)

This is a lib for working with typed physical quantities.
It ensures at compile time that you couldn't add meters to seconds or do other weird stuff.

```rust
use typed_phy::IntExt;

let length = 20.m() + 4.m();
let time = 2.s() * 3;

let speed = length / time;

assert_eq!(speed, 4.mps());
```

## Warning

WIP