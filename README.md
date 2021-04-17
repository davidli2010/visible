# visible

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](https://github.com/davidli2010/visible)
[![Crate](https://img.shields.io/crates/v/visible.svg)](https://crates.io/crates/visible)
[![Documentation](https://docs.rs/visible/badge.svg)](https://docs.rs/visible)

[![License: Apache](https://img.shields.io/badge/License-Apache%202.0-red.svg)](LICENSE-APACHE)
OR
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)

Attributes to override the visibility of items.

## Example

```Rust
#[visible::StructFields(pub(crate))]
pub struct Test {
    pub a: i32,
    pub b: i64,
}
```

The struct `Test` will be rewritten as below:

```Rust
pub struct Test {
    pub(crate) a: i32,
    pub(crate) b: i64,
}
```

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your
option. This file may not be copied, modified, or distributed
except according to those terms.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `visible` by you, shall be licensed as Apache-2.0 and MIT, without any additional
terms or conditions.
