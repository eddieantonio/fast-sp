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
<summary>Python 3.8+ with NumPy</summary>
The test cases were generated using NumPy shenanigans.

You probably want to create a virtual environment, and install NumPy
inside it. Here's one way to do it:

    python3 -m venv .venv
    source .venv/bin/activate
    pip install -r requirements.txt

</details>

# Running the benchmarks

**Note**: the `cargo bench` **must** be run before profiling Python (I'm
sorry).

Benchmark C and Rust:

    cargo bench

Benchmark Python (ensure NumPy is installed):

    python3 python/benchmark-python.py

## "Data analysis"

If you want to try analyzing results, you will need to install
additional Python packages. In your virtual environment, run this:

    python3 -m pip install -r requirements-dev.txt

This was really janky. So I just copy-pasted the output from the
benchmarks straight from my terminal into a file called `output.txt`,
but I guess you can do this:

    cargo bench | tee output.txt &&\
      python3 python/benchmark-python.py | tee -a output.txt

And then run the script to parse this file:

   python3 ./analyze-data-from-cargo-bench-output.py output.txt

# Implementations

 - `c_original` — the original implementation from the [blog post][blog].
 - `c_for_loop` — a straightforward C implementation, with buffer size given (no need to find the null-terminator).
 - `c_while_loop` — a slight variation on the original.
 - `rust_for_loop` — a Rust implementation that uses a `for` loop and mutable state.
 - `rust_iter` — a Rust implementation that uses a `for` loop and mutable state.
 - `rust_simd` — a Rust implementation that uses [Portable SIMD][].
 - `python_for_loop` — Python code to analyze buffer byte-by-byte.
 - `python_numpy` — solution that uses NumPy.

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

**TODO!** Briefly, Clang generates code for `c_original` that does all of its
counting logic with branches, which is probably why it struggles with
`random_sp` — the M1 just can't predict the branches. `c_for_loop` and
`rust_for_loop` both yields assembly that uses the `cinc` and `csel`, avoiding
costly unpredictable branches. Rust/LLVM is able to autovectorize
`rust_iter` but it generates a whole mess of instructions, and I haven't
put the time to understand what the instructions are actually doing. For
`rust_simd`, Rust/LLVM generates nice SIMD code that processes 16 bytes
at a time, in a tiny loop.

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
