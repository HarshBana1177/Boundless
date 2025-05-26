#![no_main]
risc0_zkvm::guest::entry!(main);

use risc0_zkvm::guest::env;

fn main() {
    let secret: u32 = env::read();  // private input
    let result = secret * 2;
    env::commit(&result);           // public output
}
