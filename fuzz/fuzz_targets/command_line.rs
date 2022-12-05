#![no_main]

use libfuzzer_sys::fuzz_target;
use routinator::config;

fuzz_target!(|data: Vec<&str>| {
        let _ = config::test::process_basic_args(&data);
        let _ = config::test::process_server_args(&data);
});