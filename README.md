# About

This application compares IOTA hashfunctions, namely various Curl and Troika versions:

* Curl
* Functional Curl
* BCT-Curl (batch size = 64)
* BCT-Curl (batch size = 128)
* Functional BCT-Curl (batch size = 64)
* Functional BCT-Curl (batch size = 128)
* SIMD Functional BCT-Curl (batch size = 64)
* Troika 

## What does BCT stand for?

BCT stands for **B**inary en**c**oded **t**rinary, and it is a vectorization mechanism similar to SIMD that can - given some preconditions - be used to optimize IOTA's hashing functions. It basically does the following:

* Encodes each trit using 2 bits so that for each trit only 1 bit is wasted,
* Using a 64bit integer interlaces 64 transactions and hash them in parallel.

## How to run the application
* If you haven't already, install Rust now by following the instructions [here](https://rust-lang.org/tools/install).
* Clone the repo and cd into it
* Then simply type:
    ```Bash
    cargo run --release
    ```

## TODO
* Fix the SIMD version, so it at least does produce the correct hash again (during "optimizations" I somewhere messed up :/)

## Finally

If you have any questions, don't hesitate to contact me on the IOTA Discord server (/alex/#6323). If someone finds ways to improve the code, especially if someone finds ways to further optimize any of the implementations, I'll gladly accept pull requests.

