use methods::{DOUBLE_ID, DOUBLE_PATH};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    let secret = 21;

    // Set up the zkVM environment with private input
    let env = ExecutorEnv::builder()
        .write(&secret)
        .build()
        .unwrap();

    // Prove
    let prover = default_prover();
    let receipt = prover.prove(env, DOUBLE_PATH).unwrap();

    // Verify
    receipt.verify(DOUBLE_ID).unwrap();

    // Output the public result
    let output: u32 = receipt.journal.decode().unwrap();
    println!("Public output: {}", output);  // Should print 42
}
