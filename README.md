# `typechain`

This crate provides procedural macros for generating chains of related traits in Rust.

## Example

### `types.rs`

```rust
use typechain::{chainlink, Chain};

#[chainlink]
pub trait Currency {
  fn usd_value(&self) -> f64;
}

#[derive(Chain)]
pub struct Fiat {
  #[chain(Currency)]
  pub usd_value: f64
}

#[derive(Chain)]
pub struct Crypto {
  #[chain(Currency)]
  pub usd_value: f64
}
```

### `main.rs`

```rust
use typechain::use_chains;
mod types;

use types::{Fiat, Crypto};
use_chains![types::Currency];

fn main() {
  let usd = Fiat { usd_value: 1.0 };
  let btc = Crypto { usd_value: 10000.0 };

  let currencies: Vec<&Currency> = vec![&usd, &btc];
}
```
