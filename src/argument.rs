use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::{Python};
use pyo3::types::{PyTuple, PyDict, PyAny};
use pyo3::{IntoPy, PyObject};


#[pyfunction]
fn make_the_call(py: Python, pyfunc: PyObject) -> PyResult<()> {
    let py_result : PyObject = pyfunc.call0(py)?;
    let s: String = py_result.extract(py)?;

    println!("Got string '{}'", s);
    Ok(())
}


#[pyfunction(kwds="**")]
#[text_signature = "(hello)"]
fn py_kwargs(py: Python, required_arg : u32, kwds: Option<&PyDict>) -> PyResult<()> {
    println!("required arg is {}", required_arg);
    if let Some(dict) = kwds {
        // key and value are both &PyAny
        for (key, value) in dict.iter() {
            println!("{:?} = {:?}", key, value);
        }
    }
    Ok(())
        //let py_result : PyObject = pyfunc.call0(py)?;
        //let s: String = py_result.extract(py)?;
        //println!("Got string '{}'", s);
        //let retval : PyObject = 123u32.to_object(py);
        //Ok(retval)
}


//#[pyfunction("py_defaultvalue", n = 123)]
#[pyfunction(n = 123)]
pub fn py_defaultvalue(py: Python, n: u32) -> PyResult<()> {
    println!("received n = {}", n);
    Ok(())
}
/*
*/


#[pyfunction(args = "*")]
pub fn py_varargs(py: Python, args: Option<&PyTuple>) -> PyResult<()> {
    if let Some(args) = args {
        for (i,arg) in args.iter().enumerate() {
            println!("arg{} == {:?}", i, arg);
        }
    }
    Ok(())
}

