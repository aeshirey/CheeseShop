use pyo3::prelude::*;
use pyo3::types::PyAny;

/// Instructor of defense against fresh fruit
///
/// Note that `#[pyclass] can only be used with C-style structs`, so even though
/// this example has no fields, we can't make this a unit struct.
#[pyclass]
#[derive(Default)]
pub struct Instructor;

#[pymethods]
impl Instructor {
    #[new]
    pub fn new() -> Self {
        Self
    }

    /// Defends against an attack by a student
    #[pyo3(signature = (student_obj))]
    pub fn defend(&self, student_obj: &Bound<'_, PyAny>) -> PyResult<()> {
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
                "basket of raspberries" => "Instructor releases a tiger",
                _ => "Instructor releases a tiger",
            }
        );

        Ok(())
    }

    fn __repr__(&self) -> PyResult<&'static str> {
        Ok("<Instructor>")
    }
    fn __str__(&self) -> PyResult<&'static str> {
        self.__repr__()
    }
}

/// Represents a student and their weapon of choice
#[pyclass]
pub struct Student {
    // Ideally, we'd use a Rust enum, but I'm not sure how we can expose an enum properly to Python
    // One of:
    // "Banana", -- Mr Harrison/Apricot; gets shot
    // "Raspberry", -- Mr Thompson/Tinned Peach; 16 ton weight
    // "Basket of Raspberries" -- Remaining students; tiger
    weapon: String,
}

#[pymethods]
impl Student {
    /// The `__init__` method
    #[new]
    fn new(weapon: String) -> Self {
        Student { weapon }
    }

    /// Attack with fresh fruit
    fn attack(&self) {
        println!("Student attacks with a {}", self.weapon);
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<Student wielding a {}>", self.weapon))
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test_weapon_name() {
        let mr_apricot = self_defense::Student::new("banana".to_string());
        assert_eq!(mr_apricot.weapon, "banana".to_string());
    }
}
