# dtree

Simle library crate that provides a directory tree implementation and a tiny
bit of operating sysstem state.

# Assignment Writeup

The hardest part of this assignement was actually trying to wrap my head around
the some of the asssignment requirements, specifically the `paths` and `with_subdir`
functions. Once I understood what I was trying to do it was fairly straight
forward to implement. I am actually constantly amazed by how the compiler enforces
correctness in your code. Basically, once I wrote something that `rustc` would let
me compile, the code was complete and correct. After which, all that was left was
writing the tests. I wrote each test to essentially exercise a specific function
and it's edge cases. They ended up looking very similar to the provided
examples in the doc strings.

Rusts iterators, and pattern matching makes writing expressive and correct code
a breeze and I'm growing to really enjoy programming in the language. As an extra
I was planning on implementing a "mini" operating system that used the `dtree`
implementation from this homework. However, I completely ran out of time and did
not get to work on it.
