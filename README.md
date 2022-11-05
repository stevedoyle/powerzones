# powerzones

A utility library for calculating cycling power zones using [Hunter Allen's formula](https://www.hunterallenpowerblog.com/2015/05/power-training-zones-101.html).


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
powerzones = "0.1.0"
```

## Example

```rust
let ftp: u32 = 200;
let zones = calc_power_zones(ftp);
```

## License

Rand is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).