# CheeseShop
Examples of using [PyO3 Rust bindings for Python](https://github.com/pyo3/pyo3) with little to no silliness. For detailed documentation, see the [PyO3 user guide](https://pyo3.rs/v0.19.2). This project uses the current version (0.25) as of 2025-05-22.

## Getting Started
* Create your library project: `$ cargo new --lib CheeseShop`

Update your `Cargo.toml`:

```none
[lib]
name = "CheeseShop"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.25", features = ["extension-module"] }
```

Here, `name` sets the name of the output library. In Linux, this compiles to `libCheeseShop.so`. This must be renamed to `CheeseShop.so`, which lets you `import CheeseShop` in Python.

## Running Code in This Project
There's an [`example_usage.py`](example_usage.py) file that invokes the Rust methods. For example:

```python
from CheeseShop import *
>>> do_something()
['And', 'now', 'for', 'something', 'completely', 'different']
>>> movies()
[('Monty Python and the Holy Grail', 1975), ('Life of Brian', 1979), ('The Meaning of Life', 1983)]
```

## Writing Rust functions

Some simple examples of Rust functions that become available in Python:

```rust
use pyo3::prelude::*;

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
fn CheeseShop(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(do_something))?;
    m.add_wrapped(wrap_pyfunction!(movies))?;
}
```

The `///` Rust [doc comments](https://doc.rust-lang.org/reference/comments.html#doc-comments) turn into Python docstrings, so calling `help(movies)` for the above function will show you this in the Python REPL:

```text
Help on built-in function movies:

movies()
    A module-level function that simply returns tuples of movies and their release year.
```

## Attributes
PyO3 provides numerous [attributes](https://doc.rust-lang.org/reference/attributes.html) for conveniently mapping your Rust code to comparable Python functionality:

### Python Classes
* `#[pyclass]` marks a struct (or fieldless enum) as a Python class
* `#[pymethods]` marks _one_ `impl` block for that struct (or enum). Functions/methods are generally accessible in Python, and they don't need `pub` modifiers on them.
* `#[new]` makes a Rust function (within the `pymethods` block) into the `__init__` constructor for a class
* Magic (or sometimes called 'dunder') methods are declared like other methods. Your implementation will need the approprate number and type of arguments and return values; for example:
    * `fn __setattr__(&self) {}` will fail to compile with the error, _Expected 2 arguments, got 0_ because [`setattr`](https://docs.python.org/3/library/functions.html#setattr) requires two inputs.
    * `fn __setattr__(&self, name: i32, value: i32) {}` will fail at runtime with the error, _argument 'name': 'str' object cannot be interpreted as an integer_ because the `name` argument is of an incorrect type
    * `fn __setattr__(&self, name: i32, value: i32) -> bool {}` will fail to compile because this magic method is expected to have no return value (or, technically, [`unit`](https://doc.rust-lang.org/std/primitive.unit.html)).
* `#[staticmethod]` marks an associated Rust function as being a Python static method

### Signatures
By default, methods and functions have basic [help documentation](https://docs.python.org/3/library/functions.html#help) that includes argument names. You can use `#[pyo3(text_signature = "...")]` to override this auto-generated text; the provided string is arbitrary and is simply appended to the end of the function name; it is expected that your `text_signature` encloses argumens in parentheses; eg, `(name: str, age: int = None)`.

To supply your own default values, to handle `*args` and `**kwargs`, and more, use `#[pyo3(signature = (...))]`. The argument names in this attribute _must_ match the names used in the Rust function it annotates, and the default values (if provided) must be valid Rust code that corresponds to theinput type.
