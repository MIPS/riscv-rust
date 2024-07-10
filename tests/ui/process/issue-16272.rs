//@ run-pass
//@ ignore-wasm32 no processes
//@ ignore-sgx no processes
//@ ignore-riscv64-mti-linux-gnu-cross-compile

use std::process::Command;
use std::env;

fn main() {
    let len = env::args().len();

    if len == 1 {
        test();
    } else {
        assert_eq!(len, 3);
    }
}

fn test() {
    let status = Command::new(&env::current_exe().unwrap())
                         .arg("foo").arg("")
                         .status().unwrap();
    assert!(status.success());
}
