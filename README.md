# advent-of-code
Advent of Code solutions in Rust

## How to compile
This project has been setup with Nix.
Provided you have Nix installed, you can get the development tools running by entering the Nix flake's shell:
```bash
nix develop
```

Once that has finished running, compilation can be done with:
```bash
cargo build
```

## How to run
Running the program can be done using `cargo` in the `nix develop` shell:
```bash
cargo run -- --year YEAR --puzzle DAY --file ./path/to/input.txt
```
