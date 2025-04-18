# powerzones

A utility library and binary for calculating cycling power zones using [Hunter Allen's method](https://www.hunterallenpowerblog.com/2015/05/power-training-zones-101.html).


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
powerzones = "0.2.0"
```

## Example

```rust
let ftp: u32 = 200;
let zones = calc_power_zones(ftp);
```

## Binary

```
cargo run 200
```

or

```
powerzones 200
```

## Tests

```
cargo test
```

## Contributing

1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin my-new-feature`
5. Submit a pull request :D

## License

Powerzones is distributed under the terms of the MIT license.
