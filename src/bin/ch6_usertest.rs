#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static TESTS: &[&str] = &[
    "ch4_sleep\0",
    "ch4_sleep1\0",
    "ch5_mmap0\0",
    "ch5_mmap1\0",
    "ch5_mmap2\0",
    "ch5_mmap3\0",
    "ch5_unmap\0",
    "ch5_unmap2\0",
    "ch5_spawn0\0",
    "ch5_spawn1\0",
    "ch6_mail0\0",
    "ch6_mail1\0",
    "ch6_mail2\0",
    "ch6_mail3\0",
];

use user_lib::{spawn, waitpid};

/// 辅助测例，运行所有其他测例。

#[no_mangle]
pub fn main() -> i32 {
    for test in TESTS {
        println!("Usertests: Running {}", test);
        let pid = spawn(*test);
        let mut xstate: i32 = Default::default();
        let wait_pid = waitpid(pid as usize, &mut xstate);
        assert_eq!(pid, wait_pid);
        println!(
            "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
            test, pid, xstate
        );
    }
    println!("ch6 Usertests passed!");
    0
}
