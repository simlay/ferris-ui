#[derive(Debug)] // needed to be able to return TestResult from #[test] fns
pub enum TestError {}
pub type TestResult = Result<(), TestError>;
use std::fmt::{Display, Debug};
impl<T: Display + Debug> From<T> for TestError {
    #[track_caller]
    fn from(value: T) -> Self {
        panic!("error: {value:?}")
    }
}

#[test]
fn simple() {
    assert_eq!(2, 1 + 1);
}
