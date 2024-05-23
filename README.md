# Build things with L-Systems
![Github tests](https://github.com/TheRiver/rusty-systems/actions/workflows/rust.yml/badge.svg)
![Docs.rs documentation](https://img.shields.io/docsrs/rusty-systems)

`rusty-systems` is a crate for procedurally generating content using L-Systems. It can be installed
from https://crates.io/crates/rusty-systems:

```shell
cargo add rusty-systems
```

This project is still in its early days and might undergo rapid changes. See the 
[crate docs][docs] for more information on how to use this library.


Here is a brief (and trivial) example of using the library

```rust
use rusty_systems::prelude::*;

let system = System::new();
system.parse_production("CompanyName -> Surname Surname").unwrap();

let starting_axiom = system.parse_prod_string("CompanyName").unwrap();
let result = system.derive(starting_axiom, RunSettings::default()).unwrap().unwrap();

println!("The resulting string is:\n{}", system.format(&result).unwrap());

```

## Documentation

* The main documentation for this project is available at [docs.rs][docs].
* A [changelog][changelog] is also available. 
* The website for this project is available at https://theriver.github.io/rusty-systems/

## Installation

The most recent released version of the crate can be installed by adding it to your projects `crate.toml` file:

```shell
cargo add rusty-systems
```

You can also install any of the tagged development versions using git: 

```toml
[dependencies]
rusty-systems = {git = "https://github.com/TheRiver/rusty-systems.git", tag = "0.3.0"}

```

## License

This code is licensed under the MIT license. See the [LICENSE][license]
file for more details.

[docs]: https://docs.rs/rusty-systems/latest/rusty_systems/
[license]: https://github.com/TheRiver/rusty-systems/blob/main/LICENSE
[changelog]: https://github.com/TheRiver/rusty-systems/blob/main/CHANGELOG.md
