#![allow(non_snake_case)]
use pyo3::{
    exceptions,
    prelude::*,
    types::{PyBool, PyDict, PyFloat, PyFunction, PyInt, PyList, PyNone, PySet, PyString, PyTuple},
    wrap_pyfunction, BoundObject,
};

mod cheese_shop;
mod self_defense;

/// Does something completely different by returning a Python `List[str]`.
#[pyfunction]
fn do_something() -> Vec<&'static str> {
    "And now for something completely different"
        .split(' ')
        .collect()
}

/// A module-level function that simply returns tuples of movies and their release year.
#[pyfunction]
fn movies() -> Vec<(String, u16)> {
    vec![
        ("Monty Python and the Holy Grail".to_string(), 1975),
        ("Life of Brian".to_string(), 1979),
        ("The Meaning of Life".to_string(), 1983),
    ]
}

/// Accepts an optional bool argument. Current implementation throws a TypeError if a non-bool is
/// passed.
///
/// >>> are_we_arguing()
/// >>> are_we_arguing(False)
/// >>> are_we_arguing(True)
// Sets the default value of an argument if it's not passed from Python.
// This function also _can_ get the Python GIL marker, but it doesn't use it here.
#[pyfunction]
#[pyo3(signature = (having_an_argument = false))]
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
#[pyfunction]
#[pyo3(signature = (client_says = String::from("No you haven't.")))]
fn ive_told_you_once(client_says: Option<String>) -> PyResult<String> {
    let response = match client_says.as_deref().unwrap_or("No you haven't.") {
        "No you haven't." => "Yes I have.",
        "You didn't!" => "I did!",
        "When?" => "Just now.",
        "You most certainly did not!" => "I most definitely told you!",
        _ => {
            return Err(exceptions::PyValueError::new_err(
                "I'm not allowed to argue any more.",
            ))
        }
    };
    Ok(response.to_string())
}

/// Accepts Python **kwargs.
///
/// >>> knights_at_camelot(Bedevere='Wise', Lancelot='Brave', Galahad='Pure', Robin='Not Quite so Brave as Sir Lancelot')
#[pyfunction]
#[pyo3(signature = (**kwargs))]
fn knights_at_camelot(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<()> {
    if let Some(dict) = kwargs {
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
#[pyfunction]
#[pyo3(signature = (*args))]
fn things_that_float(_py: Python<'_>, args: &Bound<'_, PyTuple>) -> PyResult<()> {
    println!("What also floats in water?");
    for arg in args.iter() {
        // Each arg is a &PyAny, which you can .extract() as any type and check for success.
        // Here, I handle strings and ignore others
        if let Ok(s) = arg.extract::<&str>() {
            println!("   \"{s}!\"");
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
#[pyo3(signature = (pyfunc))]
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
#[pyo3(signature = (callable, max_i, max_j))]
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
                return Err(exceptions::PyValueError::new_err(
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
#[pyo3(signature = (pyfunc))]
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

/// Given some object, print out its type.
#[pyfunction]
#[pyo3(signature = (hiding_behind))]
fn how_not_to_be_seen(hiding_behind: &Bound<'_, PyAny>) -> PyResult<()> {
    let r#type = if hiding_behind.extract::<&str>().is_ok() {
        "string"
    } else if hiding_behind.extract::<i32>().is_ok() {
        "integer"
    } else if hiding_behind.extract::<f32>().is_ok() {
        "float"
    } else if hiding_behind.extract::<bool>().is_ok() {
        "bool"
    } else if hiding_behind.extract::<Bound<'_, PyList>>().is_ok() {
        "list"
    } else if hiding_behind.extract::<Bound<'_, PySet>>().is_ok() {
        "set"
    } else if hiding_behind.extract::<Bound<'_, PyDict>>().is_ok() {
        "dictionary"
    } else if hiding_behind.extract::<Bound<'_, PyFunction>>().is_ok() {
        "function"
    } else if hiding_behind.extract::<Bound<'_, PyModule>>().is_ok() {
        "module"
    } else {
        println!("How not to be seen: {hiding_behind:?}");
        return Ok(());
    };

    println!(
        "Mr. Nesbitt has chosen a very obvious piece of cover behind that {}",
        r#type
    );

    Ok(())
}

/// Attempt to confuse the Python interpreter by asking it to return a value of a specified type.
#[pyfunction]
#[pyo3(signature = (result_type))]
fn confuse(py: Python, result_type: &str) -> PyResult<Py<PyAny>> {
    let result_type = result_type.to_lowercase();
    let r = match result_type.as_ref() {
        "int" => PyInt::new(py, 123u32).into(),
        "float" => PyFloat::new(py, 6.18).into(),
        "str" => PyString::new(py, "It's").into(),
        "bool" => PyBool::new(py, true).into_bound().into(),
        "none" => PyNone::get(py).into_bound().into(),
        "list" => {
            let list = PyList::empty(py);
            list.append("It's")?;
            list.append(6.18)?;
            list.append(true)?;
            list.append(PyNone::get(py))?;
            list.into_any().into()
        }
        "tuple" => {
            let items: [Py<PyAny>; 5] = [
                PyInt::new(py, 123u32).into(),
                PyFloat::new(py, 6.18).into(),
                PyString::new(py, "It's").into(),
                PyBool::new(py, true).into_bound().into(),
                PyNone::get(py).into_bound().into(),
            ];
            PyTuple::new(py, items)?.into_bound().into()
        }
        "dict" => {
            let dict = PyDict::new(py);
            dict.set_item("int", 123u32)?;
            dict.set_item("float", 6.18)?;
            dict.set_item("str", "It's")?;
            dict.set_item("bool", true)?;
            dict.set_item("none", PyNone::get(py))?;
            dict.into_any().into()
        }
        "set" => {
            let set = PySet::new(py, [1, 2, 3])?;
            set.add(4)?;
            set.add("It's")?;
            set.add(true)?;
            set.into_any().into()
        }
        // TODO: module, function?
        rt => {
            return Err(exceptions::PyValueError::new_err(format!(
                "Meow. Not sure what to do with {rt}"
            )))
        }
    };

    Ok(r)
}

/// A Python module implemented in Rust, inspired by Monty Python's Flying Circus
#[pymodule]
fn CheeseShop(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // The CheeseShop type (from cheese_shop.rs) is exported here.
    m.add_class::<cheese_shop::CheeseShop>()?;
    m.add_class::<self_defense::Student>()?;
    m.add_class::<self_defense::Instructor>()?;

    // Add functions
    // We can either add_wrapped:
    m.add_wrapped(wrap_pyfunction!(do_something))?;
    m.add_wrapped(wrap_pyfunction!(movies))?;
    // or add_function:
    m.add_function(wrap_pyfunction!(are_we_arguing, m)?)?;
    m.add_function(wrap_pyfunction!(ive_told_you_once, m)?)?;
    m.add_function(wrap_pyfunction!(knights_at_camelot, m)?)?;
    m.add_function(wrap_pyfunction!(things_that_float, m)?)?;
    m.add_function(wrap_pyfunction!(make_the_call, m)?)?;
    m.add_function(wrap_pyfunction!(call_with_args, m)?)?;
    m.add_function(wrap_pyfunction!(call_with_tuple_arg, m)?)?;
    m.add_function(wrap_pyfunction!(how_not_to_be_seen, m)?)?;

    m.add_function(wrap_pyfunction!(confuse, m)?)?;
    Ok(())
}
