use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use pyo3::wrap_pyfunction;
use pyo3::Python;

mod cheese_shop;
use cheese_shop::CheeseShop;

mod self_defense;
use self_defense::{Instructor, Student};

/*
A few comments before we get started:
 * Use #[text_signature(foo, bar)] to present the function signature to Python.
   It must go after #[pyfunction], and there are limitations on characters that can go in.
 * /// comments will show up in the Python help() for methods.
 *

Some TODO items:
 * Add some details on downcast_ref (soon to be downcast)
 * Understand how memory is handled when passing objects back and forth.
 * Document all of the 'dunder' methods currently available, create a good example of them.
 * Figure out how to wrap_pyfunction!() a function defined in another module. (This currently fails.)
 */

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

/* ----------------------------------------------------------------------------------------------
                          "I'd like to have an argument, please."
---------------------------------------------------------------------------------------------- */
// The following functions are related to passing arguments to Rust -- *args, **kwargs, default
// values, etc.
//
// Note that these really should go into another file, arguments.rs. However, I found that doing
// that gave the following compiler error (nightly-2020-03-12-x86_64-unknown-linux-gnu):
//
//    |     m.add_wrapped(wrap_pyfunction!(py_defaultvalue))?;
//    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
//
// So for now, I'm mashing all these functions into this file.
//

/// Accepts an optional bool argument. Current implementation throws a TypeError if a non-bool is
/// passed.
///
/// >>> are_we_arguing()
/// >>> are_we_arguing(False)
/// >>> are_we_arguing(True)
// Sets the default value of an argument if it's not passed from Python.
#[pyfunction(having_an_argument = false)]
#[text_signature = "(having_an_argument = False)"]
fn are_we_arguing(_py: Python, having_an_argument: bool) -> &'static str {
    if having_an_argument {
        "If I argue with you, I must take up a contrary position!"
    } else {
        "Yes, but it isn't just saying \"No, it isn't.\""
    }
}

/// Have an argument with a client. The client says something (or not), and this function responds.
/// quotes and is itself enquoted.
///
/// >>> ive_told_you_once()
/// >>> ive_told_you_once("No you haven't.")
/// >>> ive_told_you_once("When?")
// Note that the default string value needs to contain escaped quotes and is itself enquoted.
#[pyfunction(client_says = "\"No you haven't.\"")]
#[text_signature = "(client_says = \"No you haven't.\")"]
fn ive_told_you_once(_py: Python, client_says: &str) -> PyResult<&'static str> {
    match client_says {
        "No you haven't." => Ok("Yes I have."),
        "You didn't!" => Ok("I did!"),
        "When?" => Ok("Just now."),
        "You most certainly did not!" => Ok("I most definitely told you!"),
        _ => Err(pyo3::exceptions::PyValueError::new_err(
            "I'm not allowed to argue any more.",
        )),
    }
}

/// Accepts Python **kwargs.
///
/// >>> knights_at_camelot(Bedevere='Wise', Lancelot='Brave', Galahad='Pure', Robin='Not Quite so Brave as Sir Lancelot')
#[pyfunction(kwds = "**")]
#[text_signature = "(**kwargs)"]
fn knights_at_camelot(_py: Python, kwds: Option<&PyDict>) -> PyResult<()> {
    if let Some(dict) = kwds {
        println!("King Arthur's has the following knights at his round table:");
        // key and value are both &PyAny
        for (key, value) in dict.iter() {
            let adjective: String = value.extract().unwrap();
            println!("  Sir {}, the {}", key, adjective);
        }
    } else {
        println!("King Arthur has no knights. Perhaps that Black Knight guarding the bridge?");
    }

    Ok(())
}

/// Accepts a variable number of string arguments on input.
///
/// >>> things_that_float("Bread", "Apples", "Very small rocks", "Cider")
/// >>> things_that_float("Great gravy", "Churches", "Lead")
/// >>> things_that_float("A duck")
#[pyfunction(args = "*")]
#[text_signature = "(**kwargs)"]
pub fn things_that_float(_py: Python, args: Option<&PyTuple>) -> PyResult<()> {
    println!("What also floats in water?");
    if let Some(args) = args {
        for arg in args.iter() {
            // Each arg is a &PyAny, which you can .extract as you want. For this example, I'm
            // assuming everything is a string.
            println!("   \"{}!\"", arg.extract::<String>().unwrap());
        }
    }

    Ok(())
}

/// Pass a callable object from Python and invoke it in Rust.
///
/// >>> make_the_call(lambda: "Bloody vikings")
/// >>> make_the_call(lambda: 42)
/// >>> make_the_call(lambda: 1.618)
#[pyfunction]
#[text_signature = "(pyfunc)"]
fn make_the_call(py: Python, pyfunc: PyObject) -> PyResult<()> {
    let py_result: PyObject = pyfunc.call0(py)?;

    if let Ok(strval) = py_result.extract::<String>(py) {
        println!("Got a string value: '{}'", strval);
    } else if let Ok(intval) = py_result.extract::<i32>(py) {
        println!("Got an integral value: {}", intval);
    } else if let Ok(floatval) = py_result.extract::<f32>(py) {
        println!("Got an real value: {}", floatval);
    } else {
        println!("Got some other type of value: {:?}", py_result);
    }
    Ok(())
}

/// This is markedly less silly than other examples.
/// Pass a function from Python to Rust, then have Rust call the Python function.
///
/// We expect `callable` to take two numeric arguments and return a bool. Rust will invoke
/// `callable` for all (i,j) pairs for i from 1..max_i and j from i..max_j. If your function
/// returns true, Rust will write a line.
#[pyfunction]
#[text_signature = "(callable, max_i, max_j)"]
fn call_with_args(py: Python, callable: PyObject, max_i: u8, max_j: u8) -> PyResult<()> {
    for i in 1..max_i {
        for j in i..max_j {
            let args = (i, j);
            let py_result: PyObject = callable.call1(py, args)?;
            if let Ok(boolval) = py_result.extract::<bool>(py) {
                if boolval {
                    println!("func({}, {}) is true", i, j);
                }
            } else {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "func({}, {}) didn't return a bool!",
                ));
            }
        }
    }

    Ok(())
}

/// Pass a function to Rust and it will call you back with the tuple ("bicycle", "repair", "man").
/// Whatever you return will be printed to the console.
#[pyfunction]
#[text_signature = "(callable)"]
fn call_with_tuple_arg(py: Python, pyfunc: PyObject) -> PyResult<()> {
    let arg_tuple = ("bicycle", "repair", "man");
    let py_result: PyObject = pyfunc.call1(py, (arg_tuple,))?;
    if let Ok(strval) = py_result.extract::<String>(py) {
        println!("Your function returned the string '{}'", strval);
    } else {
        println!("Your function returned some non-string value");
    }
    Ok(())
}

/// This module is a python module implemented in Rust.
#[pymodule]
#[allow(non_snake_case)]
fn CheeseShop(_py: Python, m: &PyModule) -> PyResult<()> {
    // The CheeseShop type (from cheese_shop.rs) is exported here.
    m.add_class::<CheeseShop>()?;

    // Module-level functions are added here. They must have the #[pyfunction] attribute.
    m.add_wrapped(wrap_pyfunction!(do_something))?;
    m.add_wrapped(wrap_pyfunction!(movies))?;

    // For having an argument.
    m.add_wrapped(wrap_pyfunction!(are_we_arguing))?;
    m.add_wrapped(wrap_pyfunction!(ive_told_you_once))?;
    m.add_wrapped(wrap_pyfunction!(knights_at_camelot))?;
    m.add_wrapped(wrap_pyfunction!(things_that_float))?;
    m.add_wrapped(wrap_pyfunction!(make_the_call))?;
    m.add_wrapped(wrap_pyfunction!(call_with_args))?;
    m.add_wrapped(wrap_pyfunction!(call_with_tuple_arg))?;

    // For passing objects back and forth
    m.add_class::<Instructor>()?;
    m.add_class::<Student>()?;
    Ok(())
}
