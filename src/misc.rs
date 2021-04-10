pub type RuntimeResult<T> = Result<T, String>;

#[macro_export]
macro_rules! to_runtime_result {
    ($expression: expr) => {
        match $expression {
            Ok(x) => Ok(x),
            Err(e) => Err(e.to_string()),
        }
    };
}

#[macro_export]
macro_rules! to_i32 {
    ($v: ident) => {
        match $v.parse::<i32>() {
            Ok(x) => Ok(x),
            Err(e) => Err(e.to_string()),
        }
    };
}
