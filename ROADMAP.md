# Roadmap

## v0.1.x
- [x] Support for Structs with no C-compatible fields (Using *mut c_void)
- [ ] Support for Struct with C-compatible fields with 0-cost solution
- [ ] Support for Structs with N generics
- [x] Support for Enums with no C-compatible fields
- [x] Support for Enums with C-compatible fields with 0-cost solution
- [x] Support for Impl blocks
- [x] Support for standalone functions
- [x] Support for async functions wiht configurable async distpacher
- [ ] Support for callbacks
- [ ] Support for slices borrowed: &[T] / &str
- [ ] Export std predule as part of the ezffi crate (Option, Result...)
- [ ] Export std types as part of the ezffi crate behind a feature
- [ ] Export std functions as part of the ezffi crate behind a feature
- [ ] Export tokio types as part of the ezffi crates behind a feature
- [ ] Export tokio functions as part of the ezffi crate behind a feature
- [x] Support for config file (ezffi.toml) with parsing and generation instructions
- [ ] Allow crates to depend on ezffi and another crate that, at the same time, depends on ezffi and exports symbols the first crate wants to use
- [ ] Make the crate work with stable toolchain