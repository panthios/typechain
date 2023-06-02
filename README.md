# `typechain`

This crate provides procedural macros for generating chains of related traits in Rust.

## Example

### `types.rs`

```rust
use typechain::{chainlink, chain};

chainlink!(Currency => {
  const usd_value: f64;
});

chain!(Fiat => {
  @Currency
  const usd_value: f64;
});

impl Fiat {
  pub fn new(usd_value: f64) -> Self {
    Self { usd_value }
  }
}

chain!(Crypto => {
  @Currency
  const usd_value: f64;
});

impl Crypto {
  pub fn new(usd_value: f64) -> Self {
    Self { usd_value }
  }
}
```

### `main.rs`

```rust
use typechain::use_chains;
mod types;

use types::{Fiat, Crypto};
use_chains![types::Currency];

fn main() {
  let usd = Fiat::new(1.0);
  let btc = Crypto::new(10000.0);

  let currencies: Vec<&Currency> = vec![&usd, &btc];
}
```
