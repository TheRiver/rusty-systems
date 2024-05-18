# Build things with a grammar
![Testings](https://github.com/TheRiver/rusty-systems/actions/workflows/rust.yml/badge.svg)

This project is still in its early days and might undergo rapid changes. See the 
[crate docs][docs] for more information on how to use this library.

Here is a brief example 

```rust
use rusty_systems::prelude::*;

let system = System::new();
system.parse_production("CompanyName -> Surname Surname").unwrap();

let starting_axiom = system.parse_prod_string("CompanyName").unwrap();
let result = system.derive(starting_axiom, RunSettings::default()).unwrap().unwrap();

println!("The resulting string is:\n{}", system.format(&result).unwrap());

```

## Examples

Skia plant:

```shell
cargo run --example skia-plant
```

## Documentation

* The main documentation for this project is available at [docs.rs][docs].
* A [changelog][changelog] is also available.

## License

This code is licensed under the MIT license. See the [LICENSE][license]
file for more details.

[docs]: https://docs.rs/rusty-systems/latest/rusty_systems/
[license]: https://github.com/TheRiver/rusty-systems/blob/main/LICENSE
[changelog]: https://github.com/TheRiver/rusty-systems/blob/main/CHANGELOG.md