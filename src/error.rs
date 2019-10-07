use std::fmt;
use std::hash::{Hash};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::borrow::Borrow;
pub trait ValidationError {
    fn new() -> Self where Self: Sized;
    fn num_errors(&self) -> usize;
    fn merge(&mut self, error: &Self) -> usize;
    fn has_errors(&self) -> bool {
        self.num_errors() > 0
    }
    fn is_ok(&self) -> bool {
        !self.has_errors()
    }
}

pub trait ValidationErrors: ValidationError
    where <<Self as ValidationErrors>::Concern as ToOwned>::Owned: Hash + Eq {
    type Concern: ToOwned;
    type Error: ValidationError;
    fn items(&self) -> &HashMap<<<Self as ValidationErrors>::Concern as ToOwned>::Owned, Self::Error>;
    fn report_error(&mut self, concern: &<<Self as ValidationErrors>::Concern as ToOwned>::Owned, error: &Self::Error) -> usize;
    fn report_result<V>(
        &mut self, concern: &Self::Concern, result: &Result<V, &Self::Error>
    ) -> usize {
        if let Err(ref error) = result {
            self.report_error(&concern.to_owned(), error.clone())
        } else {
            0
        }
    }
}
#[derive(Clone, Debug)]
pub struct BasicValidationError{}
impl ValidationError for BasicValidationError {
    fn new() -> BasicValidationError {
        BasicValidationError{}
    }
    fn num_errors(&self) -> usize {
        1
    }
    fn merge(&mut self, _: &BasicValidationError) -> usize {
        0
    }
}
// https://users.rust-lang.org/t/unexpected-type-inferred-as-hashmap-key/29089
pub struct Validation<C: Hash + Eq + ToOwned + Debug, E: ValidationError>
    where C::Owned: Hash + Eq + Clone {
    items: HashMap<C::Owned, E>,
    num_errors: usize
}
impl <C: Hash + Eq + ToOwned + Debug, E: ValidationError> Validation<C, E>
    where C::Owned: Hash + Eq + Clone {
    pub fn get_or_create_item(&mut self, concern: &C::Owned) -> &mut E {
        self.items.entry(concern.clone()).or_insert(
            E::new()
        )
    }
}
impl <C: Hash + Eq + ToOwned + Debug, E: ValidationError> ValidationError for Validation<C, E>
    where C::Owned: Hash + Eq + Clone {
    fn new() -> Validation<C, E> {
        let items: HashMap<C::Owned, E> = HashMap::new();
        Validation{items, num_errors: 0}
    }
    fn num_errors(&self) -> usize {
        self.num_errors
    }
    fn merge(&mut self, validation: &Validation<C, E>) -> usize {
        let mut count = 0;
        for (ref concern, ref other) in &validation.items {
            count += self.report_error(concern, other)
        }
        count
    }
}
impl <C: Hash + Eq + ToOwned + Debug, E: ValidationError> ValidationErrors for Validation<C, E>
    where C::Owned: Hash + Eq + Clone {
    type Concern = C;
    type Error = E;
    fn items(&self) -> &HashMap<C::Owned, E> {
        &self.items
    }
    fn report_error(
        &mut self, concern: &C::Owned, error: &E
    ) -> usize {
        if error.is_ok() { return 0; }
        let is_new = !self.items.contains_key(concern.borrow());
        let proper: &mut E = self.get_or_create_item(concern);
        let num_merged = proper.merge(&error);
        let count = if is_new {
            error.num_errors()
        } else {
            num_merged
        };
        self.num_errors += count;
        count
    }
}
impl <C: Hash + Eq + ToOwned + Debug, E: ValidationError> std::fmt::Debug for Validation<C, E>
    where C::Owned: Hash + Eq + Clone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "Validation found {} errors", self.num_errors)
    }
}
#[derive(Debug, Clone)]
pub enum Error {
    OutOfBoundsError(String),
    NotANumberError(String),
    TooManyEdgesError,
    NullEdgeError,
    FatalError(String),
    TooManyPathsError
}
impl Error {
    pub fn out_of_bounds(
        validation: &Validation<&'static str, BasicValidationError>
    ) -> Option<Error> {
        if validation.is_ok() {
            None
        } else {
            let parameters: Vec<String> = validation.items().
                iter()
                .map(|(concern, _)| format!("{}", concern))
                .collect();
            let string = format!("{}", parameters.join(", "));
            Some(Error::OutOfBoundsError(string))
        }
    }
    pub fn not_a_number(
        validation: &Validation<&'static str, BasicValidationError>

    ) -> Option<Error> {
        if validation.is_ok() {
            None
        } else {
            let parameters: Vec<String> = validation.items().
                iter()
                .map(|(concern, _)| format!("{}", concern))
                .collect();
            let string = format!("{}", parameters.join(", "));
            Some(Error::OutOfBoundsError(string))
        }
    }
}
impl Display for Error{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::OutOfBoundsError(string) => {
                write!(f, "Invalid parameters: {}", string)
            },
            Error::NotANumberError(string) => {
                write!(f, "NaN parameters: {}", string)
            },
            Error::TooManyEdgesError => {
                write!(f, "Too many edges")
            },
            Error::NullEdgeError => {
                write!(f, "Null edge encountered")
            },
            Error::TooManyPathsError => {
                write!(f, "Too many paths")
            },
            Error::FatalError(string) => {
                write!(f, "Fatal error: {}", string)
            }
        }
    }
}

impl std::error::Error for Error{
    fn description(&self) -> &str {
        match self {
            Error::OutOfBoundsError(_) => {
                "Some parameters out of bounds"
            },
            Error::NotANumberError(_) => {
                "Some parameters are NaN"
            },
            Error::TooManyEdgesError => {
                "Too many edges"
            },
            Error::TooManyPathsError => {
                "Too many paths"
            },
            Error::NullEdgeError => {
                "Null edge encountered"
            },
            Error::FatalError(_) => {
                "Fatal error"
            }

        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        ValidationError,
        ValidationErrors,
        BasicValidationError,
        Validation
    };

    #[test]
    fn basic_validation_error_test() {
        let mut bve0: BasicValidationError = BasicValidationError::new();
        assert_eq!(bve0.num_errors(), 1);
        assert!(bve0.has_errors());
        assert!(!bve0.is_ok());
        let bve1 = BasicValidationError::new();
        assert_eq!(bve0.merge(&bve1), 0);
        assert_eq!(bve0.num_errors(), 1);
    }

    #[test]
    fn validation_test() {
        let mut bve0: BasicValidationError = BasicValidationError::new();
        let mut val0: Validation<String, BasicValidationError> = Validation::new();
        assert_eq!(val0.num_errors(), 0);
        assert!(!val0.has_errors());
        assert!(val0.is_ok());
        val0.report_error(&"x".to_owned(), &bve0);
        val0.report_error(&"y".to_owned(), &bve0);
        assert_eq!(val0.num_errors(), 2);
        assert!(val0.has_errors());
        assert!(!val0.is_ok());

        let bve1 = BasicValidationError::new();
        bve0.merge(&bve1);
        assert_eq!(bve0.num_errors(), 1);

        let mut val1: Validation<String, BasicValidationError> = Validation::new();
        val1.report_error(&"y".to_owned(), &bve0);
        val1.report_error(&"z".to_owned(), &bve0);
        assert_eq!(val1.num_errors(), 2);
        assert_eq!(val1.merge(&val0), 1);
        assert_eq!(val1.num_errors(), 3);
    }
}