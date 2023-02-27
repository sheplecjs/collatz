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
                            Err(_) => {
                                println!("Input a valid number of bits.");
                                continue;
                            },
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

        let mut seq: BigInt = input.clone(); // for current transformation;
        let mut step: u32 = 1;

        loop {
            println!("Step {step}: {seq}");

            if seq == BigInt::from(1) {
                println!("{input} reduced to 1 in {step} steps.");
                break;
            }

            seq = collatz_sequence(seq);
            step += 1
        }
    }
}

fn collatz_sequence(n: BigInt) -> BigInt {
    if n.is_odd() {
        (n*3) + 1
    } else {
        n / 2
    }
}
