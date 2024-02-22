//! **input** is a simple utility library, which provides simple functions to read user input for cli's.
//! ## Primitives
//! ```
//! let line: String = input("> ");
//! let x: u32 = input("x = ");
//! let y: f64 = input("y = ");
//! ```
//! ## Complex
//! input can read and parse all `T` where `T` : [`FromStr`], `T::Err` : [`Display`]
//! ```
//! let addr: std::net::IpAddr = input("ip: ");
//! let email: Email = input("email: "); // Custom struct - Email
//! ```

use std::fmt::Display;
use std::io::{stdin, stdout, BufRead, Write};
use std::str::FromStr;

/// Wrapper for [`input_from`] with `writer`: [`stdout()`] and `reader`: [`stdin()`].
pub fn input<T>(msg: &str) -> T
where
    T: FromStr,
    T::Err: Display,
{
    input_from(msg, &mut stdout(), &mut stdin().lock())
}

/// **Prompts** the user for **input** with `msg`.
/// - `writer`: designates the destination for **user output**.
/// - `reader`: the **src** from which input is read and parsed.
pub fn input_from<T>(msg: &str, writer: &mut impl Write, reader: &mut impl BufRead) -> T
where
    T: FromStr,
    T::Err: Display,
{
    loop {
        prompt(writer, msg);
        match next_line(reader).trim().parse() {
            Ok(t) => return t,
            Err(err) => writeln!(writer, "{}", err).expect("Failed to write Error!"),
        }
    }
}

fn prompt(writer: &mut impl Write, msg: &str) {
    write!(writer, "{}", msg).expect("Failed to write msg!");
    writer.flush().expect("Failed flushing writer!");
}

fn next_line(reader: &mut impl BufRead) -> String {
    let mut buff = String::new();
    reader.read_line(&mut buff).expect("Failed to read input!");
    buff
}

#[cfg(test)]
mod tests {
    use std::fmt::{Debug, Display};
    use std::net::{IpAddr, Ipv4Addr};
    use std::str::FromStr;

    use super::input_from;

    #[derive(Debug, PartialEq)]
    struct TestInputResult<T> {
        output: String,
        value: T,
    }

    fn run_input_from<T>(msg: &str, input: &str) -> TestInputResult<T>
    where
        T: FromStr,
        T::Err: Display,
    {
        let mut output = Vec::<u8>::new();
        let value: T = input_from(msg, &mut output, &mut input.as_bytes());
        TestInputResult {
            output: String::from_utf8(output).expect("Error reading output!"),
            value,
        }
    }

    #[test]
    fn test_input_from_primitives() {
        assert_eq!(
            run_input_from("> ", "Hello World!"),
            TestInputResult::<String> {
                output: "> ".into(),
                value: "Hello World!".into()
            }
        );
        assert_eq!(
            run_input_from("> ", "3.14159"),
            TestInputResult::<f64> {
                output: "> ".into(),
                value: 3.14159
            }
        );
        assert_eq!(
            run_input_from("> ", "47"),
            TestInputResult::<u32> {
                output: "> ".into(),
                value: 47
            }
        );
        assert_eq!(
            run_input_from("> ", "true"),
            TestInputResult::<bool> {
                output: "> ".into(),
                value: true
            }
        );
    }

    #[test]
    fn test_input_from_complex() {
        assert_eq!(
            run_input_from("> ", "127.0.0.1"),
            TestInputResult::<IpAddr> {
                output: "> ".into(),
                value: Ipv4Addr::LOCALHOST.into()
            }
        )
    }
}
