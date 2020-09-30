# OS system statistics library for Rust
This is a library to get system metrics like cpu load and memory usage, inspired by [go-osstat](https://github.com/mackerelio/go-osstat).

## Cargo.toml
```toml
os_stat = "0.1"
```

## Example
```
extern crate os_stat;

fn main() {
    dbg!(os_stat::CPU::get());
}
```

## Supported OS
Only Linux(>= 2.6.33)

## TODOs
- [ ] Better error-handling
- [ ] Support macOS
- [ ] Support Windows
