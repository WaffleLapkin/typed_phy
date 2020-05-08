# typed_phy 
(TODO: maybe I should rename the lib?)

[![CI status](https://github.com/WaffleLapkin/typed_phy/workflows/Continuous%20integration/badge.svg)](https://github.com/WaffleLapkin/arraylib/actions)
[![Telegram](https://img.shields.io/badge/tg-WaffleLapkin-9cf?logo=telegram)](https://vee.gg/t/WaffleLapkin)
[![documentation (master)](https://img.shields.io/badge/docs-master-blue)](https://typed-phy.netlify.com/typed_phy)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)



<!--
(commented until release)
[![crates.io](http://meritbadge.herokuapp.com/typed_phy)](https://crates.io/crates/typed_phy)
[![documentation (docs.rs)](https://docs.rs/typed_phy/badge.svg)](https://docs.rs/typed_phy)
-->

This is a lib for working with typed physical quantities.
It ensures at compile time that you couldn't add meters to seconds or do other weird stuff.

```rust
use typed_phy::IntExt;

let length = 20.m() + 4.m();
let time = 2.s() * 3;

let speed = length / time;

assert_eq!(speed, 4.mps());
```

the crate isn't published on crates.io yet, so to use it spicify it as a git dependency:

```toml
[dependencies]
# Replace <...> with commit you want to use (it is recomended to use the lates commit)
typed_phy = { git = "https://github.com/WaffleLapkin/typed_phy", rev = "<...>" }
```

## Warning

WIP
