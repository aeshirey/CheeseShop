use pyo3::{prelude::*, types::PyAny};

#[pyclass]
/// Sells cheesy comestibles.
pub struct CheeseShop {
    stock: u32,
}

#[pymethods]
impl CheeseShop {
    // The __init__ method
    #[new]
    pub fn new(is_hungry: Option<bool>) -> Self {
        if Some(true) == is_hungry {
            println!("Hello hungry customer!");
        }

        CheeseShop { stock: 0 }
    }

    /// Checks whether the type of cheese specified is available.
    //#[pyo3(signature = (name = None), text_signature = "(bleh) -> foo")]
    //#[pyo3(signature = (name = None))]
    #[pyo3(text_signature = "(name: str) -> bool")]
    pub fn has_cheese(&self, name: Option<&str>) -> bool {
        match name {
            Some(s) => println!("We have no {s}"),
            None => println!("No cheese whatsoever."),
        }
        false
    }

    /// Handles all responses to clients' requests as to the whereabouts of cheese.
    /// Guaranteed to not raise ValueError, as this cheese shop is the finest in the district.
    #[pyo3(signature = (cheese_type), text_signature = "whoa, that's weird")]
    pub fn respond_to_client(&self, cheese_type: &str) -> PyResult<&str> {
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
    pub fn unavailable_cheese_count() -> PyResult<u32> {
        Ok(45)
    }

    // Python 'dunder methods'. See PyO3's PyObjectProtocol trait here
    // https://github.com/PyO3/pyo3/blob/fee755adbea01504d12cd858e2705608c10163a4/src/class/basic.rs
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("CheeseShop(stock={})", self.stock))
    }

    pub fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    /// cs = CheeseShop()
    /// assert cs.its == "Monty Python's flying Circus"
    pub fn __setattr__(&mut self, name: &str, value: &PyAny) -> PyResult<()> {
        let strval: Result<&str, PyErr> = value.extract();
        if let Ok(strval) = strval {
            println!("Setting self.{name} : str = \"{strval}\"");
        } else {
            // Other types of .extract() may be done, too.
            println!("Setting self.{name} = {value:?}");
        }

        Ok(())
    }

    /// ```python
    /// cs = CheeseShop()
    /// assert cs.cannibalism == "Relatively under control"
    /// ```
    ///
    /// References the [Lifeboat sketch](https://en.wikipedia.org/wiki/Lifeboat_sketch)
    pub fn __getattr__(&self, name: &str) -> PyResult<&'static str> {
        match name {
            "cannibalism" => Ok("Relatively under control"),
            _ => Err(pyo3::exceptions::PyAttributeError::new_err(
                "Attribute not found. Move along.",
            )),
        }
    }
}
