use std::fmt;
use std::time::Duration;

pub trait Solver {
    fn get_row_count(&self) -> usize;
    fn get_labels(&self, row: usize) -> Vec<String>;
    fn execute(&self, row: usize) -> Result;
}

pub struct Result {
    pub solution: Solution,
    pub duration: Duration,
}

#[derive(Debug)]
pub enum Solution {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    String(String),
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Solution::U8(solution) => solution.fmt(f),
            Solution::U16(solution) => solution.fmt(f),
            Solution::U32(solution) => solution.fmt(f),
            Solution::U64(solution) => solution.fmt(f),
            Solution::U128(solution) => solution.fmt(f),
            Solution::USize(solution) => solution.fmt(f),
            Solution::I8(solution) => solution.fmt(f),
            Solution::I16(solution) => solution.fmt(f),
            Solution::I32(solution) => solution.fmt(f),
            Solution::I64(solution) => solution.fmt(f),
            Solution::I128(solution) => solution.fmt(f),
            Solution::ISize(solution) => solution.fmt(f),
            Solution::String(solution) => solution.fmt(f),
        }
    }
}

// A solution is only intended to be used for printing in the results table. For this purpose, any
// solutions that convert to the same string are equal.
impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
