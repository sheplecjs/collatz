use std::io;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use std::time::Instant;
use num_bigint::{BigInt, RandBigInt};
use num_integer::Integer;
use num_traits::sign::Signed;
use polars::prelude::{DataFrame, CsvReader, CsvWriter, SerReader, SerWriter, NamedFrom};
use polars::frame::UniqueKeepStrategy::First;
use polars::df;



// #[allow(unused)]
// fn main() {
// let now = Instant::now();

// // Calling a slow function, it may take a while

// let elapsed_time = now.elapsed();
// println!("Running slow_function() took {} seconds.", elapsed_time.as_secs());
// }

fn main() {

    loop {
        println!(concat!(
            "Input a positive integer to show its Collatz sequence.\n",
            "Input 'random' to use a random integer.\n",
            "Input 'range' to test a range of integers.\n",
            "Input 'exit' to exit."));

        let mut collatz = String::new();

        io::stdin()
            .read_line(&mut collatz)
            .expect("Failed to read line");

        let now = Instant::now();
        
        let input: BigInt = match collatz.trim().parse() {
            Ok(big_int) => big_int, // specific number specified
            Err(_) => {
                match collatz.trim() {
                    "random" => { // random bits generator
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
                    "exit" => { // exit condition
                        break;
                    },
                    "range" => { // range condition
                        let mut start: BigInt;
                        let end: usize;
                        loop {
                            println!("Start:");
                            let mut s = String::new();
                            io::stdin()
                                .read_line(&mut s)
                                .expect("Failed to read line");
                            match s.trim().parse() {
                                Ok(big_int) => {
                                    start = big_int;
                                },
                                Err(_) => {
                                    println!("Invalid input.");
                                    continue;
                                }
                            }
                            println!("How many?");
                            let mut e = String::new();
                            io::stdin()
                                .read_line(&mut e)
                                .expect("Failed to read line");
                            match e.trim().parse() {
                                Ok(num) => {
                                    end = num;
                                    break;
                                },
                                Err(_) => {
                                    println!("Invalid input.");
                                    continue;
                                }
                            }
                        }
                        for x in 0..end {
                            let t: BigInt = BigInt::from(start.clone() + BigInt::from(x));
                            sequence(t, false, now);
                        }
                        continue;
                    },
                    _ => { // non-recognized command
                        continue;
                    },
                }
            },
        };

        sequence(input, true, now)
    }
}

fn sequence(n: BigInt, verbose: bool, now: Instant) {
    let mut seq: BigInt = n.clone(); // for current transformation;
    let mut step: u32 = 1;

    loop {

        if verbose {
            println!("Step {step}: {seq}");
        }

        if seq == BigInt::from(1) {
            let elapsed_time = now.elapsed();
            let secs = elapsed_time.as_secs();
            println!("{n} reduced to 1 in {step} steps. Took {secs} seconds.");
            let df_old: DataFrame = read_history();
            let new_data: DataFrame = df!("Number" => &[n.to_string()], "Steps" => &[step.to_string()]).expect("DataFrame");
            let mut df_new: DataFrame = df_old.vstack(&new_data).unwrap();
            df_new = df_new.unique(None, First).expect("DataFrame");
            let mut file = std::fs::File::create("history.csv").unwrap();
            CsvWriter::new(&mut file).finish(&mut df_new).unwrap();
            break;
        }

        seq = collatz(seq);
        step += 1
    }
}

fn collatz(n: BigInt) -> BigInt {
    if n.is_odd() {
        (n*3) + 1
    } else {
        n / 2
    }
}

fn read_history() -> DataFrame {

    if Path::new("history.csv").is_file() == false {
        let mut file: File = File::create("history.csv").expect("History file creation error");
        write!(file, "Number,Steps").expect("Issue writing headers to new history file.");
    }
    
    CsvReader::from_path("history.csv")
            .expect("File")
            .has_header(true)
            .infer_schema(Some(0)) // everything as utf8 so that BigInt cast will work
            .finish().expect("DataFrame")
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use super::*;
    
    #[test]
    fn test_sequence_odd() {
        let result_odd: BigInt = collatz(BigInt::from(7));
        assert_eq!(result_odd, BigInt::from(22));
    }

    #[test]
    fn test_sequence_even() {
        let result_even: BigInt = collatz(BigInt::from(8));
        assert_eq!(result_even, BigInt::from(4));
    }
}