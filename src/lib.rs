use std::io::Write;
use std::str::FromStr;
use std::fmt::Debug;

pub fn input<'a, T>(msg: &str) -> T
where
    T: FromStr,
    T::Err : Debug,
{
    loop {
        match read_line(msg).trim().parse() {
            Ok(t) => return t,
            Err(err) => println!("{:?}", err),
        }
    }
}

fn read_line(msg: &str) -> String {
    print!("{}", msg);
    std::io::stdout().flush().unwrap();
    let mut buff = String::new();
    std::io::stdin().read_line(&mut buff).unwrap();
    buff
}
