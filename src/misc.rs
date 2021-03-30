pub type RuntimeResult<T> = Result<T, String>;

#[macro_export]
macro_rules! try_or_bubble {
  ($expression: expr) => {
    if let Err(e) = $expression {
      return Err(e.to_string())
    }
  };
}
