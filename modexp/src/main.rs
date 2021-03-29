/// Entrypoint for modexp command line utility

use modexp::modexp;

fn main() {
    println!("Hello, world! {}\n", modexp(2, 20, 17));
}
