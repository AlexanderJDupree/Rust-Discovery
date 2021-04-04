# modexp

A command line calculator for computing modular exponentiation without overflow

## Assignment Writeup

First, my strategy was to work on the `modexp` before I worried about how to 
parse command line arguments. 

My initial implementation looked right, but failed horribly.
I essentially tried to copy the psuedocode in the assignment description and 
ended up with this disaster: 

```rust
/// Naive implementation, didn't even compile
pub fn modexp(x: u32, y: u32, m: u32) {
    if x == 0 {
        return 0;
    }
    if y == 0 {
        return 1;
    }

    let z = modexp(x, y / 2, m);
    z = (z * z) mod m;

    if y mod 2 != 0 {
        // y is odd
        z = (z * x) mod m;
    }
    return z;
}
```

Which obviously did not compile, and issued out a multitude of errors. Luckily, 
the rust compiler has great error messages and I learned fairly quickly that `mod` is
not the modulo operator and I also needed a return type. Lastly, running `cargo clippy` I
learned that you don't need a trailing return and instead can end a function with an expression. Which 
I think is pretty nifty. After making the corrections I ended up with actual rust 
code that could compile. 


```rust
/// Working implementation, but did not protect against overflow
pub fn modexp(x: u32, y: u32, m: u32) -> u32 {
    if x == 0 {
        return 0;
    }
    if y == 0 {
        return 1;
    }

    let mut z = modexp(x, y / 2, m);
    z = (z * z) % m;

    if y & 1 == 1 {
        // y is odd
        z = (z * x) % m;
    }
    z;
}
```

This looked right and passed my trivial test of `assert_eq!(modexp(2,20,17), 16)`, however when using larger 
`x`, `y`, or `m` values the program would panic with "attempt to multiply with overflow". To remedy this I 
switched the multiplies to modular multiplies and used a u64 for `z` to store the larger intermediate values. The final
`modexp` function looks like this:

```rust
/// Final modexp implementation
pub fn modexp(x: u32, y: u32, m: u32) -> u32 {
    if x == 0 {
        return 0;
    }
    if y == 0 {
        return 1;
    }

    // Use larger container for modmultiply to prevent overflow
    let m64 = m as u64;
    let mut z = modexp(x, y / 2, m) as u64;
    z = (z * z) % m64;

    if y & 1 == 1 {
        // y is odd
        z = ((z % m64) * x as u64) % m64;
    }
    z as u32
}
```

All tests passed and everything was looking good! However, the function still panics when given a value of zero for `m`.
Modular by zero is undefined behavior, so following the design of other arithmetic operations in Rust I just created a 
`checked_modexp` function that will return an Option type of None if `m` is zero. 

The final piece of the puzzle was to parse the command line arguments in `main`. Fortunately, I had experience with 
iterators in different languages so I was familiar with the `map` and `collect` operations. I used those to map the `String.parse`
function over each argument and collect the results into a `Vec<u32>`. If any of the args failed to parse or the length of the resulting
vector did not equal 3 then the program just prints a help message and exits. 

Overall, I thought this assignment to be a nice and gentle introduction to rust. The algorithm is straightforward and being able to 
write tests with `#[test]` is a pretty nifty feature. All of my tests were just precomputed values I got from Python and don't test 
`modexp` over the entire possible range of inputs. I would to see if Rust supports property based testing like `hspec` or `quickcheck` in Haskell. 
