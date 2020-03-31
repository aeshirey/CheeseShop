use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::PyObject;

/// Instructor of defense against fresh fruit
#[pyclass]
pub struct Instructor {}

#[pyproto]
impl pyo3::class::PyObjectProtocol for Instructor {
    fn __repr__(&self) -> PyResult<&'static str> {
        Ok("<Instructor>")
    }
    fn __str__(&self) -> PyResult<&'static str> {
        self.__repr__()
    }
}

#[pymethods]
impl Instructor {
    #[new]
    pub fn new() -> Self {
        Instructor {}
    }

    pub fn defend(&self, student_obj: &PyAny) -> PyResult<()> {
        // See https://github.com/PyO3/pyo3/blob/master/guide/src/class.md for info on this
        let student: PyRef<Student> = student_obj.extract()?;

        // first, the student attacks
        student.attack();

        let weapon: String = student.weapon.to_lowercase();
        println!(
            "{}",
            match &weapon[..] {
                "banana" => "Instructor shoots Mr Apricot",
                "raspberry" => "Instructor drops a 16 ton weight on Mr Tinned Peach",
                "basket of raspberries" | _ => "Instructor releases a tiger",
            }
        );

        Ok(())
    }
}

#[pyclass]
pub struct Student {
    // Ideally, we'd use a Rust enum, but I'm not sure how we can expose an enum properly to Python
    // One of:
    // "Banana", -- Mr Harrison/Apricot; gets shot
    // "Raspberry", -- Mr Thompson/Tinned Peach; 16 ton weight
    // "Basket of Raspberries" -- Remaining students; tiger
    weapon: String,
}

/// dunder methods
#[pyproto]
impl pyo3::class::PyObjectProtocol for Student {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<Student wielding a {}>", self.weapon))
    }
    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }
}

#[pymethods]
impl Student {
    // The __init__method
    #[new]
    fn new(weapon: String) -> Self {
        Student { weapon }
    }

    /// Checks whether the type of cheese specified is available.
    fn attack(&self) {
        println!("Student attacks with a {}", self.weapon);
    }
}
