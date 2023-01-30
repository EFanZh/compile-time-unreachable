#![no_std]

extern "C" {
    #[link_name = "\nError: Seeing this undefined function error means that an unreachable assertion has failed because the compiler is unable to prove it.\n"]
    fn link_error() -> !;
}

/// Calling this function will trigger a link error to indicate that the compiler has failed to prove the code is
/// unreachable.
pub fn compiler_checked_unreachable() -> ! {
    unsafe { link_error() }
}

/// In debug mode, this macro delegates to [`core::unreachable`] macro; in release mode, this macro will trigger a link
/// error if the compiler is unable to prove the code is really unreachable.
#[macro_export]
macro_rules! unreachable {
    ($($args:tt)*) => {
        if cfg!(debug_assertions) {
            ::core::unreachable!($($args)*)
        } else {
            $crate::compiler_checked_unreachable()
        }
    };
}
