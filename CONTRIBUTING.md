# Contributing to Fingerprint WASM

Thanks for taking the time to contribute!
Here you can find ways to make Fingerprint WASM better, as well as tips and guidelines.

This project and everyone participating in it is governed by the [Code of Conduct](CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code.

## How you can contribute

### Reporting an issue

If you've noticed a bug, have an idea or a question,
feel free to [create an issue](https://github.com/Steven-Qiang/fingerprint-wasm/issues/new).

Before you start, please [search](https://github.com/Steven-Qiang/fingerprint-wasm/issues) for your topic.
There is a chance it has already been discussed.

When you create an issue, please provide:
- A clear description of the issue
- Steps to reproduce (if applicable)
- Expected behavior
- Actual behavior
- Browser and OS information

### Creating a pull request

If you want to fix a bug, add a source of entropy, or make any other code contribution, please [create a pull request](https://docs.github.com/en/get-started/exploring-projects-on-github/contributing-to-a-project).

In order for us to review and accept your code contributions, please follow these rules:
- Your code quality should be at least as good as the code you modify.
- Your code style should follow the Rust style guidelines.
- All new code should be covered with tests where applicable.
- The changes should be backward compatible.
- Don't add dependencies unless necessary.
- Don't make changes unrelated to the stated purpose of your pull request.

## Working with code

This section describes how to deploy the repository locally, make changes to the code, and verify your work.

### Prerequisites

Make sure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install) (1.70+)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/) (18+)
- [pnpm](https://pnpm.io/)

### Setup

```bash
git clone https://github.com/Steven-Qiang/fingerprint-wasm.git
cd fingerprint-wasm
pnpm install
```

### Development playground

Run this command to start a playground:

```bash
pnpm build:wasm
cd playground
pnpm dev
```

Then open http://localhost:5173 in a browser.

### Code style

Follow the repository's code style:
- Rust code should follow the standard Rust style guidelines
- Use `cargo fmt` to format Rust code
- Use `cargo clippy` for linting

```bash
cd wasm
cargo fmt
cargo clippy
```

### How to build

To build the WASM module:

```bash
pnpm build:wasm
```

For optimized production build:

```bash
pnpm build:wasm:optimized
```

The files will be saved to the `dist` directory.

### How to add an entropy source

An entropy source is a function that gets a piece of data about the browser.

Entropy sources are located in the [wasm/src/sources](wasm/src/sources) directory.

Entropy sources must meet the following requirements:
- It is stable — it always or almost always produces the same value in each browser
- It is selective — it produces different values in different browsers, operating systems, or devices
- It produces no side effects
- It is fast (should complete within 1 second)

Steps to add a new entropy source:

1. Create a new Rust file in `wasm/src/sources/` (e.g., `my_source.rs`)
2. Implement the source function:

```rust
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen]
pub fn get_my_source() -> Result<JsValue, JsValue> {
    // Your implementation here
    Ok(JsValue::from_str("value"))
}
```

3. Add the module to `wasm/src/sources/mod.rs`:

```rust
mod my_source;
```

4. Register the source in the `SOURCES` array in `wasm/src/sources/mod.rs`:

```rust
SourceDefinition {
    name: "mySource",
    source: || SourceResult::Sync(my_source::get_my_source()),
},
```

## License

By contributing to Fingerprint WASM, you agree that your contributions will be licensed under the MIT License.
