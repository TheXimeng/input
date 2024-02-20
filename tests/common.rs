use std::str::FromStr;

use input::input;

#[derive(Debug)]
struct Email {
    _username: String,
    _domain: String,
    _tld: String,
}

#[derive(Debug)]
struct EmailParseError;

impl FromStr for Email {
    type Err = EmailParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (username, domain) = s.split_once("@").ok_or(EmailParseError)?;
        if domain.contains("@") {
            return Err(EmailParseError);
        }
        let (domain, tld) = domain.rsplit_once(".").ok_or(EmailParseError)?;

        let username = username.trim().to_string();
        let domain = domain.trim().to_string();
        let tld = tld.trim().to_string();

        if username.is_empty() || domain.is_empty() || tld.is_empty() {
            return Err(EmailParseError);
        }

        Ok(Self {
            _username: username,
            _domain: domain,
            _tld: tld,
        })
    }
}

#[test]
fn main() {
    loop {
        let email: Email = input("> ");
        println!("{:?}", email);
    }
}
