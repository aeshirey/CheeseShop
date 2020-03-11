use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::Python;

mod cheese_shop;
use cheese_shop::CheeseShop;

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

/// This module is a python module implemented in Rust.
#[pymodule]
fn CheeseShop(_py: Python, m: &PyModule) -> PyResult<()> {
    // The CheeseShop type (from cheese_shop.rs) is exported here.
    m.add_class::<CheeseShop>()?;

    // Module-level functions are added here. They must have the #[pyfunction] attribute.
    m.add_wrapped(wrap_pyfunction!(do_something))?;
    m.add_wrapped(wrap_pyfunction!(movies))?;

    Ok(())
}
