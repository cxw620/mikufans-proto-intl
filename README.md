# Mikufans-proto-intl

![Crates.io Version](https://img.shields.io/crates/v/mikufans-proto-intl)
![GitHub Tag](https://img.shields.io/github/v/tag/cxw620/mikufans-proto-intl)
![Crates.io Total Downloads](https://img.shields.io/crates/d/mikufans-proto-intl)

## Semver explanation

**TL, DR**
We STRONGLY recommend you to specify the exact version like `mikufans-proto-intl = "=3.17.3"`
in your `Cargo.toml` to avoid unexpected breaking changes.

---

This crate may not not respect semver rules.

Take `3.17.3` as an example.

The official version is actually `3.17.0`, with *innerVer* `3170003`.

- `3` is the major version, which is not changed by this crate.
- `17` is the minor version, which is not changed by this crate.
- `3` is the crate patch version.

  Though patch version does not change, there may still be some changes
  in the content comparing with `3170000`, so we have to take `0003` as
  the crate patch version but not `0`.

For `3.17.4+build.17106976`, the first release has some problems, so we have to increase the patch version to `4`.
