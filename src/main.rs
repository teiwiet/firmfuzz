mod analyzer;
mod harness;
mod mutator;
mod fuzzer;

use harness::Harness;
use fuzzer::Fuzzer;

fn main() {
    let harness = Harness::new(
        "./test_target",
        2000,
    );

    let seeds = vec![
        b"hi\n".to_vec(),
        b"ok\n".to_vec(),
    ];

    let mut fuzzer = Fuzzer::new(harness, "./crashes", seeds);

    println!("[*] Starting fuzzer on test_target (native)");
    println!("[*] Crashes will be saved to ./crashes/");
    println!("[*] Running 100,000 iterations...\n");

    fuzzer.run(100_000);
}