pub mod u256;

pub enum ArithmeticError<T> {
    UnderFlow(T),
    Overflow(T),
}

pub enum OperationResult<T> {
    Ok(T),
    Bounds(ArithmeticError<T>),
    Err(String),
}
