use std::fmt::Display;
use std::io::{stdin, stdout, BufRead, BufReader, Read, Write};
use std::str::FromStr;

pub fn input<T>(msg: &str) -> T
where
    T: FromStr,
    T::Err: Display,
{
    input_from(msg, &mut stdout(), &mut stdin())
}

pub fn input_from<T>(msg: &str, writer: &mut impl Write, reader: &mut impl Read) -> T
where
    T: FromStr,
    T::Err: Display,
{
    let mut test = BufReader::new(reader);
    loop {
        write!(writer, "{}", msg).expect("Failed to write msg!");
        writer.flush().expect("Failed flushing writer!");
        match test.next_line().trim().parse() {
            Ok(t) => return t,
            Err(err) => writeln!(writer, "{}", err).expect("Failed to write Error!"),
        }
    }
}

trait BufReaderExt {
    fn next_line(&mut self) -> String;
}

impl<T> BufReaderExt for BufReader<T>
where
    T: Read,
{
    fn next_line(&mut self) -> String {
        let mut buff = String::new();
        self.read_line(&mut buff).expect("Failed to read input!");
        buff
    }
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
