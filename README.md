# white-balance-rs
[![Build Status](https://travis-ci.org/lovebug356/white-balance-rs.svg?branch=master)](https://travis-ci.org/lovebug356/white-balance-rs)

Rust implementation of popular white balancing methods.

## Usage

This is an example on how to apply auto white balance with all implemented methods:

```bash
$ cargo run -- -i example.jpg -a
Auto white balancing:
	Input: example.jpg (1920x1080)
	Output: gray-world -> example-gray-world.jpg
	Output: retinex -> example-retinex.jpg
	Output: gray-retinex -> example-gray-retinex.jpg
``` 
or only for the gray-world method:
```bash
$ cargo run -- -i example.jpg -m gray-world
Auto white balancing:
	Input: example.jpg (1920x1080)
	Output: gray-world -> example-gray-world.jpg
```

More information on the current arguments that are accepted:

```bash
$ cargo run -- -h
Automatic white balance for images

USAGE:
    white-balance [FLAGS] [OPTIONS] --input <input>

FLAGS:
    -a, --all        use all methods
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --auto <auto>        white balancing auto
    -i, --input <input>      input image filename
    -o, --output <output>    output image filename
```