argon2min
========

[![()
Status](https://travis-ci.org/FauxFaux/argon2min.svg)](https://travis-ci.org/FauxFaux/argon2min)

This is a purely Rust-based library that provides both variants of the
state-of-the-art Argon2 hashing algorithm, suitable for password hashing and
password-based key derivation.

## Usage

From `examples/helloworld.rs`:

```rust
extern crate argon2min;

pub fn main() {
    let (password, salt) = ("argon2i!", "delicious salt");
    println!("argon2i_simple(\"{}\", \"{}\"):", password, salt);
    for byte in argon2min::argon2i_simple(&password, &salt).iter() {
        print!("{:02x}", byte);
    }
    println!("");
}
```

outputs:

```
argon2i_simple("argon2i!", "delicious salt"):
e254b28d820f26706a19309f1888cefd5d48d91384f35dc2e3fe75c3a8f665a6
```

There are two variants of Argon2 that differ in the manner by which reference
indices are computed during block-filling rounds. Argon2d does this in a faster
but data-dependent fashion that could be vulnerable to side-channel
[attacks][1], whereas Argon2i ("i" denoting independence from plaintext input)
works slower but is immune to such attacks and is therefore the preferred choice
for password hashing.

## TODO

- [x] Zero-on-drop trait for sensitive(s): `Matrix`
- [x] Constant-time verification API.
- [x] Benchmarks.
- [ ] Fuzz.
- [ ] Prove safety of unchecked accesses in `Block`, `Matrix`.

## Benchmarks

Our primary benchmarks are single-threaded runs of Argon2i with
default parameters against the [reference implementation][2]. In order to
compile and run this, first pull in the C sources:

```bash
$ git submodule init
$ git submodule update benches/cargon/phc-winner-argon2
```

and then benchmark with Cargo as usual:

```
$ export RUSTFLAGS='-C target-feature=+avx'
$ cargo bench --features="bench_ref"

# output trimmed for brevity

     Running target/release/versus_cargon-b5955411e1594c85

running 5 tests
test ensure_identical_hashes ... ignored
test bench_argon2min_i ... bench:   6,856,774 ns/iter (+/- 197,405)
test bench_cargon_i   ... bench:   3,856,783 ns/iter (+/- 142,580)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured
```

## References

["Argon2: The Memory-Hard Function for Password Hashing and Other
Applications"][1]

[1]: https://github.com/P-H-C/phc-winner-argon2/raw/master/argon2-specs.pdf
[2]: https://github.com/p-h-c/phc-winner-argon2
