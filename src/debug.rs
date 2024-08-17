#[macro_export]
macro_rules! dbgln {
  ($($arg:tt)*) => {
    #[cfg(debug_assertions)]
    if DEBUG {
      println!($($arg)*);
    }
  }
}