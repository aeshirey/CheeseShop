# CheeseShop
Examples of using [PyO3 Rust bindings](https://github.com/pyo3/pyo3) with little to no silliness.

## Getting Started
(As of 2020-03-10)

* Make sure you have nightly Rust: `$ rustup install nightly`
* Create your library project: `$ cargo new --lib CheeseShop`
* Then make sure your project is using nighly Rust: `$ rustup override set nightly`

The important bits in `Cargo.toml` are:

```none
[lib]
name = "CheeseShop"
crate-type = ["cdylib"]

[dependencies.pyo3]
version = "0.9.0-alpha.1"
features = ["extension-module"]
```

Here, `name` sets the name of the output library. In Linux, this creates a `libCheeseShop.so` file. This must be renamed to `CheeseShop.so`, which lets you `import CheeseShop` in Python.


## Running
There's a [`example_usage.py`](example_usage.py) file that invokes the Rust methods. For example:

```
from CheeseShop import *
>>> do_something()
['And', 'now', 'for', 'something', 'completely', 'different']
>>> movies()
[('Monty Python and the Holy Grail', 1975), ('Life of Brian', 1979), ('The Meaning of Life', 1983)]
```