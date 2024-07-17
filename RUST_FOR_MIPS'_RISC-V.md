# Rust for MIPS' RISC-V

This document briefly describes how to use a [compiletest](https://rustc-dev-guide.rust-lang.org/tests/compiletest.html) tool for testing [**Rust** with the support for **MIPS' RISC-V** target](https://github.com/MIPS/riscv-rust/tree/kingv-rust).

* * *
## Building Rust and adjusting for MIPS' RISC-V
* * *

MIPS implements a new architecture based on a RISC-V architecture. It extends the RISC-V arhitecture with new instructions described [here](https://github.com/MIPS/llvm/tree/mti/main). The CPUs that implement this new architecture are called i8500 and p8700.

The Rust compiler with the support for the MIPS' RISC-V architecture can be cloned from [here](https://github.com/MIPS/riscv-rust/tree/kingv-rust). After cloning, it can be setup, build, and installed as described in this document.

### Build and install MIPS’ RISC-V GNU Compiler Toolchain

To use MIPS’ RISC-V GNU Compiler Toolchain, you should clone it from [here](https://github.com/MIPS/riscv-gnu-toolchain/tree/mti-2023.09-01) and build it like so:

```console
git clone --recursive -b mti-2023.09-01 https://github.com/mips/riscv-gnu-toolchain
```

The `-b` option checkouts a specific branch, in our case the `mti-2023.09-01` branch. Option `--recursive` will also clone the included submodules.

Next, make a build and an install directory and position to a build directory:
```console
mkdir mips-riscv-gnu-toolchain-build && mkdir mips-riscv-gnu-toolchain-install && cd mips-riscv-gnu-toolchain-build
```

Next, add `bin/` subdirectory of the install directory (specified by `--prefix` in the configuration) to your PATH:
```console
export PATH=<MIPS_riscv-gnu-toolchain_install_dir>/bin:$PATH
```

Next, configure the build process:
```console
<MIPS_riscv-gnu-toolchain_dir>/configure \
--prefix=<MIPS_riscv-gnu-toolchain_install_dir> \
--with-arch=rv64imafdc --with-abi=lp64d \
--enable-multilib
```

`--with-arch` and `--with-abi` options specify the default architecture and ABI, respectively.
These can be adjusted accordingly.
`--enable-multilib` specifies multilib configuration. 
Alternatively, `--disable-multilib` disables multilib configuration.

If out of tree GCC build should be used, add the `--with-gcc-src=<path/to/gcc>` option to the
configure command.

Finally, build the riscv-gnu-toolchain (linux target uses glibc, rather than newlib):
```console
make linux -j <thread-number>
```

Afterwards, the qemu submodule should be build as described on the corresponding github page. During the configuration of the qemu, you should specify the RISC-V 64 target and the option that prevents treating the warrnings as the fatal errors, as follows:
```console
../configure --disable-werror --target-list=riscv64-linux-user
```

The target `riscv64-linux-user` is used to build qemu-riscv64, which is used to test or run specific user programs on the riscv64 architecture, without the need to simulate the entire operating system.
In the moment of writing this document, the tool qemu-riscv64 is used for testing Rust for MIPS' RISC-V.

### Install rustup

**Rust** is installed and managed by the **rustup** tool. Rust has a 6-week rapid release process and supports a great number of platforms, so there are many builds of Rust available at any time. Rustup manages these builds in a consistent way on every platform that Rust supports, enabling installation of Rust from the beta and nightly release channels as well as support for additional cross-compilation targets.

In order to install rustup tool you need to run the following command:
```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Once in awhile you’ll need to update your installation by running:
```console
rustup update
```
For more information see the [rustup documentation](https://rust-lang.github.io/rustup/).

### Set default Rustup toolchain
You will most likely use **nightly** toolchain, thus you will install it and set it to be the default toolchain.
```console
rustup default nightly
```

### Get the MIPS' Rust source code
```console
git clone -b kingv-rust git@github.com:MIPS/riscv-rust.git
cd riscv-rust
```

### Setup the Rust configuration file
```console
./x setup
```

You will be asked to set an option regarding your intentions. See [config.example.toml](https://github.com/rust-lang/rust/blob/master/config.example.toml) for all the available settings and explanations of them. See [src/bootstrap/defaults](https://github.com/rust-lang/rust/tree/master/src/bootstrap/defaults) for common settings to change.

To build the compiler stages 0-2, before building, configure the build section of your config.toml like so to ensure that Rust is installed from source:
```toml
profile = "dist"
```

To produce a compiler that can cross-compile for the target riscv64-mti-linux-gnu, configure the build section of your config.toml like so:
```toml
[build]
target = ["x86_64-unknown-linux-gnu", "riscv64-mti-linux-gnu"]
```

It will build **stage0**, **stage1** and **stage2** compilers.
Since rust is depedent on LLVM for code generation you will also need to specify which LLVM targets to build support for.
```toml
[llvm]
targets="X86;RISCV"
```

To prevent encountering an error message 'setting targets is incompatible with download-ci-llvm' during bulding of the Rust compiler, which means that you cannot simultaneously set the LLVM targets and use the download-ci-llvm option in the config.toml file, you should modify the llvm section of config.toml like so:
```toml
[llvm]
download-ci-llvm = false
```

A C(++) compiler and archiver for the target riscv64-mti-linux-gnu must be specified, like so:
```toml
[target.riscv64-mti-linux-gnu]
cc = "<MIPS_riscv-gnu-toolchain_install_dir>/bin/riscv64-mti-linux-gnu-gcc"
cxx = "<MIPS_riscv-gnu-toolchain_install_dir>/bin/riscv64-mti-linux-gnu-g++"
ar = "<MIPS_riscv-gnu-toolchain_install_dir>/bin/riscv64-mti-linux-gnu-ar"
```

For the cross-compilation, you’ll need to add the path to the **GNU linker** for the RISC-V target.
```toml
[target.riscv64-mti-linux-gnu]
linker = "<MIPS_riscv-gnu-toolchain_install_dir>/bin/riscv64-mti-linux-gnu-gcc"
```

To enable the cross-running of the project, you need to specify the path to the **qemu** for the RISC-V target in config.toml as follows:
```toml
[target.riscv64-mti-linux-gnu]
runner = "<MIPS_riscv-gnu-toolchain_dir>/qemu/build/riscv64-linux-user/qemu-riscv64 -L <MIPS_riscv-gnu-toolchain_install_dir>/sysroot/riscv -cpu king-v"
```

### Build the Rust compiler
```console
./x build
```

### Create a rustup toolchain
Once you have successfully built **rustc**, you have created a bunch of files in your build directory. In
order to actually run the resulting rustc, you should create rustup toolchains.
```console
rustup toolchain link stage2 ./build/host/stage2
```

### Adding a new target to Rust

A tutorial for adding a new target to Rust is elaborated [here](https://rustc-dev-guide.rust-lang.org/building/new-target.html). Here is a brief explanation of what is done to add the support for the target riscv64-mti-linux-gnu in Rust.

A name of the new target and a name of the corresponding file of the new target should be added to the `supported_targets` macro in the `rustc_target::spec` module:
```rs
("riscv64-mti-linux-gnu", riscv64_mti_linux_gnu),
```

The corresponding file for the new target containing a target function should be added. You can copy `riscv64gc_unknown_linux_gnu.rs` and modify it appropriately:
```rs
llvm_target: "riscv64-mti-linux-gnu".into(),
cpu: "i8500".into(),
```

Add the unexpected `target_vendor` value to the different `Cargo.toml` files in the directory library/{std,alloc,core}.
```toml
[lints.rust.unexpected_cfgs]
check-cfg = [
    'cfg(target_vendor, values("mti"))',
]
```

To use the new target in the bootstrap, you need to explicitly add the target triple to the `STAGE0_MISSING_TARGETS` list in the file `src/bootstrap/src/core/sanity.rs` like so:
```rs
const STAGE0_MISSING_TARGETS: &[&str] = &[
    "riscv64-mti-linux-gnu",
];
```

## Compile and run hello world example

You need to export the rustup and the cargo home directory environment variables, like so:
```console
export RUSTUP_HOME='<path_to_rustup_home_directory>'
export CARGO_HOME='<path_to_cargo_home_directory>'
```

To create a new binary crate, use the following script:
```console
cargo new --bin <name>
cd <name>
```

You’ll find Cargo.toml file, which is its manifest and it contains metadata that is needed to compile the crate. See [manifest](https://doc.rust-lang.org/cargo/reference/manifest.html) for detailed information. A common way for Cargo.toml to look like is as follows:

```toml
[package]
name = "demo"
version = "0.1.0"
edition = "2021"

[profile.dev]
opt-level = 0
debug = true
debug-assertions = true
overflow-checks = true
lto = false
panic = "unwind"
incremental = true
codegen-units = 1
rpath = false
```

Then, you need to create the configuration file:
```console
mkdir -p .cargo
cd .cargo
touch config.toml
```

Adjust the configuration file as follows:
```toml
[target.riscv64-mti-linux-gnu]
linker = "<MIPS_riscv-gnu-toolchain_install_dir>/bin/riscv64-mti-linux-gnu-gcc"
runner = "<MIPS_riscv-gnu-toolchain_dir>/qemu/build/riscv64-linux-user/qemu-riscv64 -L <MIPS_riscv-gnu-toolchain_install_dir>/sysroot/riscv -cpu king-v"
```

To build and run the crate, you can use the following script:
```console
cargo +stage2 build --target riscv64-mti-linux-gnu
cargo +stage2 run --target riscv64-mti-linux-gnu
```

Additionally, you can specify the target cpu (either i8500 or p8700) as follows:
```console
RUSTFLAGS="-C target-cpu=i8500" cargo +stage2 build --target riscv64-mti-linux-gnu
RUSTFLAGS="-C target-cpu=i8500" cargo +stage2 run --target riscv64-mti-linux-gnu
``` 

* * *
## Using compiletest for testing the CPU i8500
* * *
[Compiletest](https://rustc-dev-guide.rust-lang.org/tests/compiletest.html) is the main test harness of the Rust test suite. In order to run the [UI tests](https://rustc-dev-guide.rust-lang.org/tests/ui.html), which are a particular test suite of compiletest, you should use the following command:
```console
./x test -vv --test-args --force-rerun --target riscv64-mti-linux-gnu tests/ui -- --target-rustcflags "-Ctarget-cpu=i8500"
```
The option `-vv` is used for the very verbose output.
The option `--test-args` is used for the extra arguments to be passed for the test tool being used (compiletest). The option `--force-rerun` is passed to compiletest. It is used to rerun the tests even if the inputs are unchanged.
The option `--target` is used for specifying the target targets to build.
After `--` are written the arguments passed to the subcommands. In this case, the argument `--target-rustcflags` specifies the flags to pass to the rustc for the target. There is no need to specify the target processor because the default processor for the target riscv64-mti-linux-gnu is `i8500`.

The tests that failed in the moment of writing this document are annotated with the header command `//@ ignore-riscv64-mti-linux-gnu-cross-compile`, which tells the compiletest to ignore a test if the target is riscv64-mti-linux-gnu and the host is different from that target. In the failed tests, there are problems with subprocesses on qemu-riscv64. These problems should be solved by using qemu-system-riscv64.
