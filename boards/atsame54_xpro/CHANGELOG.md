# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.0](https://github.com/atsamd-rs/atsamd/compare/atsame54_xpro-0.7.0...atsame54_xpro-0.8.0) - 2024-10-17

### Refactored

- update path of Cargo config ([#749](https://github.com/atsamd-rs/atsamd/pull/749))
- Remove build profiles from Cargo.toml ([#762](https://github.com/atsamd-rs/atsamd/pull/762))

### Dependencies

- **[breaking]** Upgrade PAC generated code to latest SVD and `svd2rust-0.34.1` [#756](https://github.com/atsamd-rs/atsamd/pull/756):
- Update HAL dependency to `0.18`

## v0.7.0

- Implement `embedded-hal` `1.0` for GPIO, SPI, I2C, UART and fix examples
- Update the PACs to svd2rust 0.30.2.

## v0.6.0
- Limit RAM memory to avoid HardFaults when `UROW:ECCRAM` is enabled
- Remove re-export of `cortex-m-rt::entry`

## v0.5.0
- update to `atsamd-hal-0.15`
- update to to `panic-semihosting-0.6`
- added functions to create all sercom devices and pads using the XPro extensions 1, 2, and 3
- Changed pin types to use their correct alternate definitions instead of using GPIO functions
- Removed the structs of pin sets which relied on old pin definitions

## v0.4.0

- update to `atsamd-hal-0.14` and other latest dependencies (#564)
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` and `panic_rtt` (#510)

---

Changelog tracking started at v0.3.0
