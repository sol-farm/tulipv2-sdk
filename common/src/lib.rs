#![deny(unused_must_use)]
#![deny(clippy::all)]
#![allow(clippy::bool_assert_comparison)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]

pub mod config;
pub mod lending;
pub mod math;
pub mod pyth;
pub mod tag;
pub mod traits;
pub mod vaults;

use anchor_lang::solana_program::pubkey::Pubkey;

pub const DEFAULT_KEY: Pubkey = Pubkey::new_from_array([
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

/// msg_panic! is a wrapper around the `msg!` and `panic!`
/// macros used to log an error message, and panic in bpf environments
/// which do not actually show a message emitted by a panic macro
#[macro_export]
macro_rules! msg_panic {
    ($($args:tt)+) => {{
        use anchor_lang::solana_program::msg;

        // the actual error message
        msg!("RUNTIME ERROR: {}", format_args!($($args)*));
        // panic to indicate the line the error came from
        // but panicking doesn't log anything in bpf target for solana programs
        panic!("RUNTIME ERROR: {}", format_args!($($args)*));
    }};
}

/// sums size of the values provided
#[macro_export]
macro_rules! sum {
    // this delcares an exrpession i think :shrug:
    // todo(): explain this more
    ($($args:expr),*) => {{
        let result = 0;
        $(
            // combine the size of each value
            let result = result + std::mem::size_of_val(&$args);
        )*
        // return the size of all arguments
        result
    }}
}

/// msg_trace! is a wrapper around the `msg!` macro, that faciliates logging trace
/// level logs, which include the file and line number from where the message was emitted.
///
/// if the total msg size is less than or equal to 512 bytes, then `arrform!` is used for
/// the optimize (heap-less) message formatting. messages larger than 512 bytes use the traditional `format!`.
#[macro_export]
macro_rules! msg_trace {
    ($($args:tt)+) => {
        use anchor_lang::solana_program::msg;
        // get the filename that produce the log, it's less info than the fille path
        // but it saves pace, an when paired with the line number is more than enough debug
        let file_name = std::path::Path::new(file!()).file_name().unwrap().to_string_lossy();
        let input_sizes = sum!($($args)*);
        if input_sizes > 512 {
            // slow path
            msg!("{}", format!("'{}', '{}:{}", format!($($args)*), file_name, line!()).as_str());
        } else {
            use tulip_arrform::{arrform, ArrForm};
            let file_info = arrform!(256, "{}:{}", file_name, line!());
            let msg_part = arrform!(512, $($args)*);
            msg!("'{}', {}", msg_part.as_str(), file_info.as_str());
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use anchor_lang::solana_program::msg;
    #[test]
    fn default_key() {
        assert_eq!(DEFAULT_KEY, Pubkey::default());
    }
    #[test]
    fn test_size_ofs() {
        println!("{}", sum!("y", "o", "bbbbbb"));
    }
    #[test]
    fn test_trace() {
        msg_trace!("hello world this is {}", "very big message");
    }
}
