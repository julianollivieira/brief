use std::{fmt::Display, str::FromStr};

use super::validation::{validate_part, InvalidPartError};

#[derive(Debug)]
pub enum ParseAddressError {
    MissingUserOrDomain,
    InvalidUser(InvalidPartError),
    InvalidDomain(InvalidPartError),
}

/// Represents an email address
///
/// You can create an `Address` from a user string and domain string:
/// ```
/// use brief::mail::Address;
///
/// let address = Address::try_new("user", "domain.com").unwrap();
/// ```
///
/// or from a string:
/// ```
/// use brief::mail::Address;
///
/// let address: Address = "user@domain.com".parse().unwrap();
/// ```
pub struct Address {
    user: String,
    domain: String,
}

impl Address {
    /// Tries to create an address from a user and domain, returning an error if the user and/or
    /// domain are invalid.
    ///
    /// ```
    /// use brief::mail::Address;
    ///
    /// let address = Address::try_new("user", "domain.com").unwrap();
    /// ```
    pub fn try_new<T: Into<String> + Copy, U: Into<String> + Copy>(
        user: T,
        domain: U,
    ) -> Result<Self, ParseAddressError> {
        validate_part(&user.into()).map_err(|e| ParseAddressError::InvalidUser(e))?;
        validate_part(&domain.into()).map_err(|e| ParseAddressError::InvalidDomain(e))?;

        Ok(Address::new_unchecked(user, domain))
    }
    /// Creates a new unchecked `Address`.
    ///
    /// ```
    /// use brief::mail::Address;
    ///
    /// let address = Address::new_unchecked("user", "domain.com");
    /// ```
    pub fn new_unchecked<T: Into<String>, U: Into<String>>(user: T, domain: U) -> Self {
        Self {
            user: user.into(),
            domain: domain.into(),
        }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("{}@{}", &self.user, &self.domain))
    }
}

impl FromStr for Address {
    type Err = ParseAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains('@') {
            return Err(ParseAddressError::MissingUserOrDomain);
        }

        let mut split = s.rsplitn(2, '@');
        let domain = split.next().unwrap_or("");
        let user = split.next().unwrap_or("");

        Address::try_new(user, domain)
    }
}

#[cfg(test)]
mod test {
    use super::Address;

    #[test]
    fn it_creates_an_address_from_valid_data() {
        let address = Address::try_new("user", "domain.com");
        assert!(address.is_ok());
    }

    #[test]
    fn it_fails_to_create_an_address_when_user_or_domain_is_empty() {
        let without_user = Address::try_new("", "domain.com");
        assert!(without_user.is_err());

        let without_domain = Address::try_new("name", "");
        assert!(without_domain.is_err());
    }

    #[test]
    fn it_fails_to_create_an_address_when_user_or_domain_contain_forbidden_characters() {
        let without_user = Address::try_new("", "@domain.com");
        assert!(without_user.is_err());

        let without_domain = Address::try_new("(name)", "");
        assert!(without_domain.is_err());
    }

    #[test]
    fn it_creates_an_address_from_a_valid_string() {
        let address = "user@domain.com".parse::<Address>();
        assert!(address.is_ok());
    }

    #[test]
    fn it_fails_to_create_an_address_from_an_invalid_string() {
        let address = "userdomain.com".parse::<Address>();
        assert!(address.is_err());
    }

    #[test]
    fn it_formats_the_address_correctly() {
        let address = Address::try_new("user", "domain.com").unwrap();
        assert_eq!(address.to_string(), "user@domain.com");
    }
}
