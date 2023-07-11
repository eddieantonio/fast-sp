# Fast SP

Various implementations of the problem in [this blog post][blog].

[blog]: https://owen.cafe/posts/six-times-faster-than-c/

# Requirements

To run this, you will need **Rust Nightly** and **Python 3.8+** with **numpy**.

<details>
<summary>Rust (nightly)</summary>
Use [rustup](https://www.rust-lang.org/tools/install) to install a Rust toolchain, then install a nightly
toolchain:

    rustup update -- nightly

Then run `rustup override set nightly` to use the Rust nightly in the
current directory.

</details>

<details>
<summary>Python 3.8+ with numpy</summary>
The test cases were generated using numpy shenanigans.

You probably want to create a virtual environment, and install numpy
inside it. Here's one way to do it:

    python3 -m venv .venv
    source .venv/bin/activate
    pip install -r requirements.txt

</details>

# Implementations

 - `count_owen_c` — the original implementation from the [blog post][blog].
 - `count_c` — a straightforward C implementation.
 - `count_for_loop` — a Rust implementation that uses a `for` loop and
   mutable state.
 - `count_iter` — a Rust implementation that uses iterators.
 - `count_simd` — a Rust implementation that uses [Portable SIMD][].

[Portable SIMD]: https://github.com/rust-lang/portable-simd

# Benchmarks

There were two test cases that I used:

 - `random_printable`: 12 MiB of random printable ASCII characters. 
 - `random_sp`: 12 MiB of either the ASCII character `s` or `p`.

I chose 12 MiB as the size of the test data, as that is the size of the
L2 cache on the M1's performance cores (allegedly).

Here's how fast various implementation strategies work on my machine (from fastest to slowest):

| Language   | Implementation   | Test case         |  Throughput (GiB/s) |                   Time per iteration |
|:-----------|:-----------------|:------------------|--------------------:|-------------------------------------:|
| Rust       | portable\_simd   | random\_sp        |              19.874 |        589,654 ns/iter ±       5,769 |
| Rust       | portable\_simd   | random\_printable |              19.870 |        589,766 ns/iter ±       5,726 |
| Python     | numpy            | random\_printable |               5.800 |      2,020,549 ns/iter ±      28,156 |
| Python     | numpy            | random\_sp        |               5.711 |      2,052,063 ns/iter ±     113,433 |
| Rust       | iter             | random\_printable |               3.979 |      2,944,916 ns/iter ±      80,495 |
| Rust       | iter             | random\_sp        |               3.978 |      2,946,037 ns/iter ±      33,849 |
| Rust       | emulate\_numpy   | random\_sp        |               3.031 |      3,866,700 ns/iter ±     116,109 |
| Rust       | emulate\_numpy   | random\_printable |               3.026 |      3,872,550 ns/iter ±     103,355 |
| C          | while\_loop      | random\_printable |               1.487 |      7,878,845 ns/iter ±     105,742 |
| C          | while\_loop      | random\_sp        |               1.486 |      7,886,922 ns/iter ±     158,662 |
| C          | for\_loop        | random\_sp        |               1.482 |      7,905,779 ns/iter ±      40,898 |
| Rust       | for\_loop        | random\_sp        |               1.482 |      7,906,754 ns/iter ±     635,075 |
| Rust       | for\_loop        | random\_printable |               1.481 |      7,912,558 ns/iter ±     499,821 |
| C          | original         | random\_printable |               1.478 |      7,930,400 ns/iter ±     135,758 |
| C          | for\_loop        | random\_printable |               1.478 |      7,931,450 ns/iter ±     516,610 |
| C          | original         | random\_sp        |               0.278 |     42,119,291 ns/iter ±     501,957 |
| Python     | for\_loop        | random\_printable |               0.001 | 18,939,466,916 ns/iter ±  44,605,333 |
| Python     | for\_loop        | random\_sp        |               0.001 | 19,451,341,325 ns/iter ± 291,675,532 |

# Analysis

**TODO!** Briefly, Clang generates code for `count_c_owen` that does all of its
counting logic with branches, which is probably why it struggles with
`random_sp` — the M1 just can't predict the branches. `count_c` and
`count_for_loop` both yields assemly that uses the `cinc` and `csel`, avoiding
costly unpredictable branches. Rust/LLVM is able to autovectorize `count_iter`
but it generates a whole mess of instructions, and I haven't put the time to
understand what the instructions are actually doing.
For `count_simd`, Rust/LLVM generates nice SIMD code that processes 16 bytes at
a time, in a tiny loop.

Details of my testing machine
 - Apple MacBook Pro M1, 2020. 
 - RAM: 8 GB
 - Max Clock speed: 3.2 GHz
 - L2 cache: 12 MB
 - Apple clang version 14.0.0 (clang-1400.0.29.202)
 - rustc 1.68.0-nightly (61a415be5 2023-01-12)
 - LLVM version: 15.0.6

# License

Copyright © 2023 Eddie Antonio Santos. AGPL-3.0 licensed.
