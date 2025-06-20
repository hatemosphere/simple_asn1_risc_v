//! Simple no_std logger for simple-asn1-nostd

#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {{
        #[cfg(feature = "debug")]
        {
            extern crate alloc;
            use alloc::format;
            // In no_std, we just format the string
            // The actual output mechanism depends on the target environment
            let _ = format!($($arg)*);
        }
    }};
}

pub use debug_log;
