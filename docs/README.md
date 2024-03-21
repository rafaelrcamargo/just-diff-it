# `jdi` or Just diff it! ✅

A straightforward terminal diff tool in Rust for quickly comparing text files and discovering differences with ease.

## Installation

Using `cargo`:

```bash
git clone https://github.com/rafaelrcamargo/just-diff-it
cd just-diff-it
cargo build --release
```

> If you want, you can now `cp target/release/jdi /usr/local/bin` to make it available system-wide.

## Usage

This is as simple as it gets. Just run `jdi` followed by the two texts you want to compare:

```bash
jdi
```

This will prompt you to input the first text, once you're done, press `Return` and write `EOF` so the program knows you're done. Then, it will ask for the second text and do the same.

After that, it will display the differences between the two texts, colored and split in two panes for easy comparison.

## License

This project is licensed under [The Unlicense](https://unlicense.org/), check the [LICENSE](LICENSE) file for more details. (tl;dr: do whatever you want with it! 🎉)
