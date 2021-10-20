<div align="center">

# `injrs`

**DLL injector library and tool written in Rust. Rust 实现的DLL注入工具/库**

[![CI](https://github.com/jiusanzhou/injrs/actions/workflows/ci.yml/badge.svg)](https://github.com/jiusanzhou/injrs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/dll-syringe.svg)](https://crates.io/crates/dll-syringe)
[![Documentation](https://docs.rs/dll-syringe/badge.svg)](https://docs.rs/dll-syringe)
[![dependency status](https://deps.rs/repo/github/jiusanzhou/injrs/status.svg)](https://deps.rs/repo/github/jiusanzhou/injrs)
[![MIT](https://img.shields.io/crates/l/dll-syringe.svg)](https://github.com/jiusanzhou/injrs/blob/master/LICENSE)

</div>

## Install

Go to [releases page](releases) download the latest binary.

Or if you have rust installed, use cargo to install:
```bash
cargo install injrs
```

Install rust if you don't have.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Usage

At most time, you can use `injrs` as a simple tool.

```bash
USAGE:
injrs PROCESS_NAME/PID [Libraies...]

EXAMPLES:
1. Inject test.dll to process Calc.exe
    $ injrs Calc.exe test.dll

2. Inject test.dll and demo.dll to process with PID: 1888
    $ injrs 1888 test.dll demo.dll
```

The code in [example](./example) is a simple message box dll for testing injector.

## Usage as library

You also can write a injector project using `injrs` as a library.

```rust
use injrs::process_windows::*;
use injrs::inject_windows::*;

fn main() {
    let name = "Calc.exe";
    let dll = "./my-demo-dll.dll";
    let p = Process::find_first_by_name(name).unwrap();

    print!("inject dll to process => ");
    match process.inject(dll) {
        Err(e) => {
            println!("error: {}", e);
        },
        Ok(_) => {
            println!("success");
        }
    }
}
```

for more detail you can check [src/main.rs](./src/main.rs).