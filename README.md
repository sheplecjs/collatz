# Collatz

## Description

The Collatz conjecture, also known as the *3n+1* problem, seeks to prove that any positive integer will be reduced to 1 when subjected to a simple set of transformations. Given an even number, the next number in the sequence is that number divided by 2. Given an odd number, the next number is that number times three plus one.

This project aims to define a CLI for interactively exploring the Collatz conjecture.

## Installation

Running and/or building from source requires a stable version of Rust and Cargo.

`cargo run` - to run the main CLI interface.

`RUSTFLAGS="-C target-cpu=native" cargo build --release` - to compile an optimized binary.

## Usage

The CLI will prompt for a positive integer to run through a Collatz sequence. It is also possible to input 'random', in which case you will be prompted to input the number of digits desired. Steps in the sequence are shown as they are calculated. Upon completion, the total steps are printed to the console and you are returned to the starting prompt. An input of 'exit' will exit the program.

For example:

```console
Input a positive integer to show its Collatz sequence.
Input 'random' to use a random integer.
Input 'range' to test a range of integers.
Input 'exit' to exit.
```
`foo@bar:~$ 10`
```console
Step 1: 10
Step 2: 5
Step 3: 16
Step 4: 8
Step 5: 4
Step 6: 2
Step 7: 1
10 reduced to 1 in 7 steps.
Input a positive integer to show its Collatz sequence.
Input 'random' to use a random integer.
Input 'range' to test a range of integers.
Input 'exit' to exit.
```
`foo@bar:~$ random`
```console
How many bits?
```
`foo@bar:~$ 4`
```console
Step 1: 12
Step 2: 6
Step 3: 3
Step 4: 10
Step 5: 5
Step 6: 16
Step 7: 8
Step 8: 4
Step 9: 2
Step 10: 1
12 reduced to 1 in 10 steps.
Input a positive integer to show its Collatz sequence.
Input 'random' to use a random integer.
Input 'range' to test a range of integers.
Input 'exit' to exit.
```
`foo@bar:~$ exit`

## License

MIT