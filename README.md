# IOTA Block PoC

This is a Rust-based proof of concept demonstrating how to connect to a rust node, add a tagged data block, and retrieve added data block using the [IOTA SDK](https://github.com/iotaledger/iota-sdk)

## Requirements

This project requires `Rust` and `Cargo`. You can find installation instructions in
the [Rust documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html).

### Dependencies

You must also install `cmake`, `clang`, and `openssl`. You may need to install additional build tools on your system to
run the build process successfully using Cargo.

#### Windows

You can download `cmake` from the [official website](https://cmake.org/download/). You can install `openssl`
with [vcpkg](https://github.com/microsoft/vcpkg) or [chocolatey](https://chocolatey.org/).

- Installing `openssl` with `vcpkg`:

```
./vcpkg.exe install openssl:x64-windows
./vcpkg.exe integrate install
# You may want to add this to the system environment variables since you'll need it to compile the crate
set VCPKGRS_DYNAMIC=1
```

- Installing `openssl` with `chocolatey`:

```
choco install openssl
# You may need to set the OPENSSL_DIR environment variable
set OPENSSL_DIR="C:\Program Files\OpenSSL-Win64"
```

#### macOS

You can install `cmake` and `openssl` with [`Homebrew`](https://brew.sh/):

```
brew install cmake openssl@1.1
```

#### Linux

You can install `cmake`, `clang`, and `openssl` with your distro's package manager or download them from their websites.
On Debian and Ubuntu, you will also need the `build-essential` and `libudev-dev` packages.

## Execution

You can use the following command to run the PoC:

```bash
cargo cargo run --release --all-features  main
```
