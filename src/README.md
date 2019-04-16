# Various Curl implementations in Rust

## Description
This projects includes various Curl implementations:
* Curl (standard)
* Curl (stateless)
* BCT-Curl (standard, 64bit) <-- Identical to the Java version
* BCT-Curl (standard, 128bit)
* BCT-Curl (stateless, 64bit) <-- the fastest implementation
* BCT-Curl (stateless, 128bit)
* SIMD-BCT-Curl (stateless, 64bit)

Interestingly the SIMD version is not the fastest implementation. Although I tried hard to optimize the code to make it faster, I think it might be very well the case, that the compiler already vectorizes the non-SIMD code automatically, and coding it manually only introduces inefficiencies. Also the 128 bit versions are slower than the 64 bit versions.

## BCT-what?

BCT-Curl is vastly more efficient than the standard Curl implementation as you'll see for yourself if you run the program. In short it optimizes on two ends:
* Encodes each trit with 2 bits instead of 8, so for each trit there is only 1 bit wasted,
* Interlaces 64 transactions and hashes them at once. It achieves this by storing the trit data in two lanes (high, low) consisting of 8019 * 64bit integers.

## How to run it
* If you haven't already, install Rust now.
* Clone the repo and cd into the project directory.
* Then simply type:
    ```Bash
    cargo run --release
    ```

## TODO
* Fix the SIMD version, so it at least does produce the correct hash again (during "optimizations" I somewhere messed up :/)

## Finally

If you have any questions, don't hesitate to contact me on the IOTA Discord server (/alex/#6323). If someone finds ways to improve the code, especially if someone finds ways to further optimize any of the implementations, I'll gladly accept pull requests.

