use std::io;
use num_bigint::{BigInt, RandBigInt};
use num_integer::Integer;
use num_traits::sign::Signed;

fn main() {

    loop {
        println!(concat!(
            "Input a positive integer to show its Collatz sequence.\n",
            "Input 'random' to use a random integer.\n",
            "Input 'exit' to exit."));

        let mut collatz = String::new();

        io::stdin()
            .read_line(&mut collatz)
            .expect("Failed to read line");

        let input: BigInt = match collatz.trim().parse() {
            Ok(big_int) => big_int,
            Err(_) => {
                match collatz.trim() {
                    "random" => {
                        let d: BigInt;
                        loop {
                        println!("How many bits?");
                        let mut digits = String::new();
                        io::stdin()
                            .read_line(&mut digits)
                            .expect("Failed to read line");
                        match digits.trim().parse() {
                            Ok(num) => {
                                let mut rng = rand::thread_rng();
                                d = rng.gen_bigint(num).abs();
                                break;
                            }
                            Err(_) => continue,
                        };
                    };
                    d
                    },
                    "exit" => {
                        break;
                    },
                    _ => {
                        continue;
                    },
                }
            },
        };

        collatz_sequence(input);

    }
}

fn collatz_sequence(n: BigInt) {

    let mut collatz_seq: BigInt = n.clone();

    let mut step: u32 = 1;

    loop {
        println!("Step {step}: {collatz_seq}");
                if collatz_seq == BigInt::from(1) {
                    println!("{n} reduced to 1 in {step} steps.");
                    break;
                } else if collatz_seq.is_odd() {
                    collatz_seq = (collatz_seq*3) + 1;
                } else {
                    collatz_seq = collatz_seq / 2;
                }
        step += 1
        }
}
