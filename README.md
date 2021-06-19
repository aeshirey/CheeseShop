# CheeseShop
Examples of using [PyO3 Rust bindings for Python](https://github.com/pyo3/pyo3) with little to no silliness.

## Getting Started
* Create your library project: `$ cargo new --lib CheeseShop`

Update your `Cargo.toml`:

```none
[lib]
name = "CheeseShop"
crate-type = ["cdylib"]

[dependencies.pyo3]
version = "0.13.2"
features = ["extension-module"]
```

Here, `name` sets the name of the output library. In Linux, this creates a `libCheeseShop.so` file. This must be renamed to `CheeseShop.so`, which lets you `import CheeseShop` in Python.

## Writing Rust functions

```rust
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
/// Does something completely different by returning a Python `List[str]`.
fn do_something() -> Vec<&'static str> {
    "And now for something completely different"
        .split(" ")
        .collect()
}

#[pyfunction]
/// A module-level function that simply returns tuples of movies and their release year.
fn movies() -> Vec<(String, u16)> {
    vec![
        ("Monty Python and the Holy Grail".to_string(), 1975),
        ("Life of Brian".to_string(), 1979),
        ("The Meaning of Life".to_string(), 1983),
    ]
}

#[pymodule]
fn CheeseShop(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(do_something))?;
    m.add_wrapped(wrap_pyfunction!(movies))?;
}
```

## Running
There's an [`example_usage.py`](example_usage.py) file that invokes the Rust methods. For example:

```
from CheeseShop import *
>>> do_something()
['And', 'now', 'for', 'something', 'completely', 'different']
>>> movies()
[('Monty Python and the Holy Grail', 1975), ('Life of Brian', 1979), ('The Meaning of Life', 1983)]
```
