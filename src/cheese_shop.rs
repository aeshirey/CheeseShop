use pyo3::exceptions::ValueError;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::PyObject;

#[pyclass]
/// Sells cheesy comestibles.
pub struct CheeseShop {
    stock: u32,
}

/// Python 'dunder methods'. See PyO3's PyObjectProtocol trait here
/// https://github.com/PyO3/pyo3/blob/fee755adbea01504d12cd858e2705608c10163a4/src/class/basic.rs
#[pyproto]
impl pyo3::class::PyObjectProtocol for CheeseShop {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("CheeseShop(stock={})", self.stock))
    }
    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    /// cs = CheeseShop()
    /// assert cs.its == "Monty Python's flying Circus"
    fn __setattr__(&mut self, name: &str, value: &PyAny) -> PyResult<()> {
        let strval: Result<&str, PyErr> = value.extract();
        if let Ok(strval) = strval {
            println!("Setting self.{} : str = \"{}\"", name, strval);
        } else {
            // Other types of .extract() may be done, too.
            println!("Setting self.{} = {:?}", name, value);
        }

        Ok(())
    }

    /// cs = CheeseShop()
    /// assert cs.cannibalism == "Relatively under control"
    fn __getattr__(&self, name: &str) -> PyResult<&'static str> {
        match name {
            "cannibalism" => Ok("Relatively under control"),
            _ => Err(ValueError::py_err("Attribute not found. Move along.")),
        }
    }
}

#[pymethods]
impl CheeseShop {
    // The __init__method
    #[new]
    fn new(is_hungry: Option<bool>) -> Self {
        if let Some(_h @ true) = is_hungry {
            println!("Hello hungry customer!");
        }
        CheeseShop { stock: 0 }
    }

    /// Checks whether the type of cheese specified is available.
    fn has_cheese(&self, name: Option<&str>) -> bool {
        match name {
            Some(s) => println!("We have no {}", s),
            None => println!("No cheese whatsoever."),
        }
        false
    }

    /// Handles all responses to clients' requests as to the whereabouts of cheese.
    /// Guaranteed to not raise ValueError, as this cheese shop is the finest in the district.
    fn respond_to_client(&self, cheese_type: &str) -> PyResult<&str> {
        let cheese_lower: String = cheese_type.to_lowercase();
        let response = match &cheese_lower[..] {
            "red leicester" => "I'm afraid we're fresh out of Red Leicester sir.",
            "tilsit" => {
                "Never at the end of the week, sir. Always get it fresh first thing on Monday."
            }
            "bel paese" | "stilton" => "Sorry.",
            "red windsor" => "Normally, sir, yes, but today the van broke down.",
            "camembert" => "Oh! The cat's eaten it.",

            _ => "No",
        };

        Ok(response)
    }

    /// A static method -- no 'self' is passed in.
    /// assert CheeseShop.unavailable_cheese_count() == 45
    #[staticmethod]
    fn unavailable_cheese_count() -> PyResult<u32> {
        Ok(45)
    }
}
