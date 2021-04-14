# toy-rsa

Simple, and very easily crackable, RSA library that provides methods for
generating RSA public/private key pairs as well as encryption and decryption of
messages.

# What's in this repo?

The library crate is in `src/` everything is self-contained in `lib.rs`. Some
basic randomized and precomputed tests are in `tests/` and an example
command-line program that exercises the 3 functions of `toy_rsa` is located in
`examples/` folder.

## Assignment Writeup

This assignment was very straightforward as the assignment background and
wikipedia article very succintly described the RSA algorithms. Since the library
was just working with numeric types I didn't have any problems from the
borrow checker. My testing strategy for this assignment was pretty straighforward
as well. I wrote some trivial tests with precomputed values and wrote a
randomized test that would generate a public/private key, generate a random message,
and encrypt -> decrypt the message and verify the results. This test runs 1000
times and if nothing breaks, then I assume everything is fine.

Lastly, as an added exercise I wrote a command line program, `toyrsa.rs` that
leverages the `toy_rsa` library crate. I wanted to get some experience with command line
parsers so I used the `clap` crate for this program and found it to be very
ergonomic. While writing this program I also picked up on some File IO basics and
learned more about the different string types in Rust like the `PathBuf` and `&Path`
types.
