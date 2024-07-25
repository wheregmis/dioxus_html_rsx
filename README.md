# Dioxus HTML RSX

This project is a Dioxus-based application with support for both server-side rendering (SSR) and web assembly (WASM) targets.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Building the Project

To build the project, run:

```sh
just run
```


## Features
Server-side Rendering (SSR): Enabled with the server feature.

Web Assembly (WASM): Enabled with the web feature.

#### Dependencies

dioxus: Core library for building user interfaces.

axum: Optional, for server-side rendering.

tokio: Optional, for asynchronous runtime.

serde: For serialization and deserialization.

manganis: Additional utilities.

#### Configuration
The project is configured using the following files:

Cargo.toml: Rust package configuration.

Dioxus.toml: Dioxus-specific configuration.

tailwind.config.js: Tailwind CSS configuration.


### License
This project is licensed under the MIT License.



