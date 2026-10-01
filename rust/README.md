# Kiota Rust Runtime Crates

Rust language runtime support for [Microsoft Kiota](https://github.com/microsoft/kiota), an OpenAPI SDK generator.

These crates provide the runtime infrastructure that Kiota-generated Rust API clients depend on.

## Crates

| Crate | Description | Status |
|:---|:---|:---|
| `kiota-abstractions` | Core traits: `RequestAdapter`, `Parsable`, `ParseNode`, `SerializationWriter` | 🔄 In progress |
| `kiota-http-reqwest` | HTTP client based on `reqwest` with middleware pipeline | 🔄 Planned |
| `kiota-serialization-json` | JSON serialization via `serde_json` | 🔄 Planned |
| `kiota-serialization-text` | Plain text serialization | 📋 Planned |
| `kiota-serialization-form` | URL-encoded form serialization | 📋 Planned |
| `kiota-serialization-multipart` | Multipart body support | 📋 Planned |
| `kiota-authentication` | Authentication providers | 📋 Planned |
| `kiota-bundle` | Convenience re-export of all crates | 📋 Planned |

## Architecture

```
Generated API Client (your code)
        │
        ▼
┌─────────────────────┐
│  kiota-abstractions  │  Core traits and types
└────────┬────────────┘
         │
    ┌────┴────┬──────────────┐
    ▼         ▼              ▼
kiota-http  kiota-json   kiota-auth
(reqwest)   (serde_json)  (providers)
```

## Building

```bash
cd rust
cargo build --all
cargo test --all
```

## Minimum Supported Rust Version

Rust 1.85.0

## Contributing

See the [coordination issue](https://github.com/kiota-community/kiota-rust/issues/18) for the current development plan.

Contributions are welcome! Please open a PR against this repository.

## License

MIT — see [LICENSE](../LICENSE)
