use std::{fmt::Display, str::FromStr};

use super::{
    address::ParseAddressError,
    validation::{validate_part, InvalidPartError},
    Address,
};

#[derive(Debug)]
pub enum ParseMailboxError {
    MissingAngleBrackets,
    MissingOpeningAngleBracket,
    MissingClosingAngleBracket,
    WrongOrderAngleBrackets,
    InvalidName(InvalidPartError),
    InvalidAddress(ParseAddressError),
}

impl From<ParseAddressError> for ParseMailboxError {
    fn from(value: ParseAddressError) -> Self {
        Self::InvalidAddress(value)
    }
}

/// Represents a mailbox
#[derive(Clone)]
pub struct Mailbox {
    name: Option<String>,
    address: Address,
}

impl Mailbox {
    /// Tries to create a mailbox from a name and address, returning an error if the name is
    /// invalid.
    ///
    /// ```
    /// use brief::mail::{Address, Mailbox};
    ///
    /// let address: Address = "user@domain.com".parse().unwrap();
    /// let mailbox = Mailbox::try_new(Some("name"), address).unwrap();
    /// ```
    pub fn try_new<T: Into<String> + Copy>(
        name: Option<T>,
        address: Address,
    ) -> Result<Self, ParseMailboxError> {
        if let Some(name) = name {
            validate_part(&name.into()).map_err(|e| ParseMailboxError::InvalidName(e))?;
        }

        Ok(Self {
            name: name.map(|v| v.into()),
            address,
        })
    }
    /// Creates a new unchecked `Mailbox`.
    ///
    /// ```
    /// use brief::mail::{Address, Mailbox};
    ///
    /// let address: Address = "user@domain.com".parse().unwrap();
    /// let mailbox = Mailbox::new_unchecked(Some("name"), address);
    /// ```
    pub fn new_unchecked<T: Into<String> + Copy>(
        name: Option<T>,
        address: Address,
    ) -> Result<Self, ParseMailboxError> {
        Ok(Self {
            name: name.map(|v| v.into()),
            address,
        })
    }
}

impl Display for Mailbox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(name) = &self.name {
            f.write_str(name)?;
            f.write_str(" ")?;
        }

        f.write_fmt(format_args!("<{}>", &self.address.to_string()))
    }
}

impl FromStr for Mailbox {
    type Err = ParseMailboxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match (s.find('<'), s.find('>')) {
            (None, Some(_)) => Err(ParseMailboxError::MissingOpeningAngleBracket),
            (Some(_), None) => Err(ParseMailboxError::MissingClosingAngleBracket),
            (Some(left), Some(right)) => {
                if left > right {
                    return Err(ParseMailboxError::WrongOrderAngleBrackets);
                }

                let (name_str, rest) = s.split_once('<').unwrap();
                let address_str = rest.split_once('>').unwrap().0;

                let name = (!name_str.is_empty()).then(|| name_str).or_else(|| None);
                let address: Address = address_str.parse()?;

                Ok(Self {
                    name: name.map(|v| v.trim().to_owned()),
                    address,
                })
            }
            (None, None) => {
                if s.contains(" ") {
                    return Err(ParseMailboxError::MissingAngleBrackets);
                }

                Ok(Self {
                    name: None,
                    address: s.parse()?,
                })
            }
        }
    }
}

/// Represents multiple mailboxes
#[derive(Clone)]
pub struct Mailboxes(Vec<Mailbox>);

impl Display for Mailboxes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter().peekable();

        while let Some(mailbox) = iter.next() {
            mailbox.fmt(f)?;

            if iter.peek().is_some() {
                f.write_str(", ")?;
            }
        }

        Ok(())
    }
}

impl FromStr for Mailboxes {
    type Err = ParseMailboxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(",")
            .map(|m| m.trim().parse::<Mailbox>())
            .collect::<Result<Vec<_>, _>>()
            .map(Mailboxes)
    }
}

impl From<Mailbox> for Mailboxes {
    fn from(value: Mailbox) -> Self {
        Mailboxes(vec![value])
    }
}

#[cfg(test)]
mod test {
    use crate::mail::Address;

    use super::{Mailbox, Mailboxes};

    #[test]
    fn it_creates_a_mailbox_from_valid_data() {
        let mailbox = Mailbox::try_new(Some("name"), "user@domain.com".parse().unwrap());
        assert!(mailbox.is_ok());
    }

    #[test]
    fn it_creates_a_mailbox_when_name_is_empty() {
        // TODO: fix this, why is type declaration needed?
        let mailbox = Mailbox::try_new::<&str>(None, "user@domain.com".parse().unwrap());
        assert!(mailbox.is_ok());
    }

    #[test]
    fn it_creates_a_mailbox_from_a_string_with_a_valid_name_and_address() {
        let mailbox = "name <user@domain.com>".parse::<Mailbox>();
        assert!(mailbox.is_ok());
    }

    #[test]
    fn it_creates_a_mailbox_from_a_string_with_a_valid_address() {
        let mailbox = "user@domain.com".parse::<Mailbox>();
        assert!(mailbox.is_ok());
    }

    #[test]
    fn it_creates_a_mailbox_from_a_string_with_a_valid_address_and_angle_brackets() {
        let mailbox = "<user@domain.com>".parse::<Mailbox>();
        assert!(mailbox.is_ok());
    }

    #[test]
    fn it_fails_when_the_brackets_are_invalid() {
        let cases = [
            "user user@domain.com>",
            "user <user@domain.com",
            "user user@domain.com",
            "user >user@domain.com<",
            "user@domain.com>",
            "<user@domain.com",
            ">user@domain.com<",
        ];

        for v in cases {
            assert!(v.parse::<Mailbox>().is_err())
        }
    }

    #[test]
    fn it_formats_the_mailbox_correctly() {
        let address: Address = "user@domain.com".parse().unwrap();
        let mailbox = Mailbox::try_new(Some("name"), address).unwrap();

        assert_eq!(mailbox.to_string(), "name <user@domain.com>");
    }

    #[test]
    fn it_formats_mailboxes_correctly_single() {
        let mailboxes = Mailboxes(vec!["name <user@domain.com>".parse().unwrap()]);

        assert_eq!(mailboxes.to_string(), "name <user@domain.com>");
    }

    #[test]
    fn it_formats_mailboxes_correctly_multiple() {
        let mailboxes = Mailboxes(vec![
            "name <user@domain.com>".parse().unwrap(),
            "nametwo <usertwo@domaintwo.com>".parse().unwrap(),
        ]);

        assert_eq!(
            mailboxes.to_string(),
            "name <user@domain.com>, nametwo <usertwo@domaintwo.com>"
        );
    }

    #[test]
    fn it_parses_mailboxes_correctly_single() {
        let mailbox = "name <user@domain.com>".parse::<Mailboxes>();

        assert!(mailbox.is_ok());
        assert_eq!(mailbox.unwrap().0.len(), 1);
    }

    #[test]
    fn it_parses_mailboxes_correctly() {
        let cases = vec![
            "name <user@domain.com>, nametwo <usertwo@domaintwo.com>",
            "name <user@domain.com>, <usertwo@domaintwo.com>",
            "<user@domain.com>, nametwo <usertwo@domaintwo.com>",
            "<user@domain.com>, <usertwo@domaintwo.com>",
            "user@domain.com, usertwo@domaintwo.com",
        ];

        for v in cases {
            assert!(v.parse::<Mailboxes>().is_ok())
        }
    }
}
