#[macro_export]
macro_rules! dbgln {
  ($($arg:tt)*) => {
    #[cfg(debug_assertions)]
    if DEBUG {
      println!($($arg)*);
    }
  }
}

#[macro_export]
macro_rules! should_debug {
  (yes) => {
    #[cfg(debug_assertions)]
    const DEBUG: bool = true;
  };

  (no) => {
    #[cfg(debug_assertions)]
    const DEBUG: bool = false;
  };

  (1) => {
    #[cfg(debug_assertions)]
    const DEBUG: bool = true;
  };

  (0) => {
    #[cfg(debug_assertions)]
    const DEBUG: bool = false;
  };

  (true) => {
    #[cfg(debug_assertions)]
    const DEBUG: bool = true;
  };

  (false) => {
    #[cfg(debug_assertions)]
    const DEBUG: bool = false;
  };

  () => {
    #[cfg(debug_assertions)]
    const DEBUG: bool = true;
  };
}