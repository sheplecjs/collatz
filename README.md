# Collatz

## Description

The Collatz conjecture, also known as the *3n+1* problem, seeks to prove that any positive integer will be reduced to 1 when subjected to a simple set of transformations. Given an even number, the next number in the sequence is that number divided by 2. Given an odd number, the next number is that number times three plus one.

This project aims to define a CLI for interactively exploring the Collatz conjecture. Included functionality:

+ Detailed sequences for specific inputs with annotated transformations
+ Steps, and computation time reporting for random bits and ranges of inputs
+ Recording of unique inputs and step values to a postgresql connection or flat file

## Installation

A Dockerfile is provided running the rust:slim-buster image.

A docker-compose file is provided which will run a postgresql db alongside the main CLI. Run these and the CLI in the current terminal with the following command:

`docker-compose -f docker-compose-collatz.yml run --rm collatz`

A seperate environment is provided where the db and jupyterlab is available for analysis of results:

`docker-compose -f docker-compose-analysis.yml up`

Running and/or building from source requires a stable version of Rust and Cargo.

`cargo run` - to run the main CLI interface.

`RUSTFLAGS="-C target-cpu=native" cargo build --release` - to compile an optimized binary.

### Note:

The release profile for cargo build is modified from defaults in the following ways which offer potentially better binary performance at the cost of increased compile time:

+ Link Time Optimization is set to 'true' (same as 'fat')
+ Codegen units are set to 1 (default for --release is 16)
+ Panic is set to 'abort' - this just fails more quickly

## Usage

At startup, the CLI will prompt for a storage option accepting either a postgres connection string or 'flat' to write to a csv file, defaulting to the postgres instance spun up by the docker-compose definiton.

The CLI will then prompt for a positive integer to run through a Collatz sequence. It is also possible to input 'random'. Steps in the sequence are shown as they are calculated. Upon completion, the total steps are printed to the console and you are returned to the starting prompt. Specifying 'range' will allow you to specify a starting point and a number of incrementing iterations. Massive numbers are supported using the BigInt struct. Using any of the provided options, unique solutions will be added to the persistent history file as they are reached, recording the input and number of steps taken. An input of 'exit' will exit the program.

For example:

```console
Input psql connection string to use db (default, postgresql://postgres:@db:5432/postgres).
Input 'flat' to use flat file.

Input a positive integer to show its Collatz sequence.
Input 'random' to use a random integer.
Input 'range' to test a range of integers.
Input 'exit' to exit.
```
`10`
```console
Step 1: 10 - (Reduce)
Step 2: 5 - (Augment)
Step 3: 16 - (Reduce)
Step 4: 8 - (Reduce)
Step 5: 4 - (Reduce)
Step 6: 2 - (Reduce)
Step 7: 1 - (Base)
10 reduced to 1 in 7 steps. Took 0 seconds.
Input a positive integer to show its Collatz sequence.
Input 'random' to use a random integer.
Input 'range' to test a range of integers.
Input 'exit' to exit.
```
`random`
```console
How many bits?
```
`4096`
```console
971726121250224429417262494028854911280777504314214600622785496787474748657674979106790234904200898615074908639963468183581338655156474899287322992232749594093090248086888945187756345998332769284985512492012025532807510622603325124488897335479103858564926491861994399092114775454129739992858202375654820473190712757642856425455714354757441001087819285337780935782150925000013202732065323472413153984997562198933505097350200691053218296804752978522550409157608219831402414189229392595327104162336547181946158178776191465412932990359139431053144402912267228889465944309513227148179761959474563130982586469297814199105233960742374942461312437046848493640979215338958403079782101352228517368276864301915683358356450235724546084333392819641224454893962965063593396896988275881702473180351614998580746272213591447097768375314833461510809847283564877744749957529441777905185091823630454247989360158169620426314606415095925642033734377283584788537846704024207402109607796986783559073267664904851245972696329313952561257436779191592280283499265048054633146760469052579045185952846764331063562618025690674419950413522932556719393614557959341086837850405712305435992861134342689459557804390426847518294350967154440487324597681258913716328858446 reduced to 1 in 29117 steps. Took 3 seconds.
Input a positive integer to show its Collatz sequence.
Input 'random' to use a random integer.
Input 'range' to test a range of integers.
Input 'exit' to exit.
```
`exit`

## License

MIT

## Next up

- Trigger early exit once a tested start is hit mid-sequence
- Implement initial check for power of 2 if length of input is large enough