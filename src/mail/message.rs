use super::{header::Header, mailbox::Mailboxes};

pub struct Message {
    headers: Vec<Header>,
    // body
}

pub struct MessageBuilder {
    headers: Vec<Header>,
    // body
}

impl MessageBuilder {
    /// Creates a new message builder.
    ///
    /// ```
    /// use brief::mail::MessageBuilder;
    ///
    /// let builder = MessageBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
        }
    }
    /// Adds a header to the message.
    ///
    /// ```
    /// use brief::mail::{Mailbox, Header, MessageBuilder};
    ///
    /// let sender: Mailbox = "name <user@domain.com>".parse().unwrap();
    /// let from_header = Header::From(sender.into());
    /// let builder = MessageBuilder::new().header(from_header);
    /// ```
    pub fn header(mut self, header: Header) {
        self.headers.push(header);
    }
    /// Convenience method for adding a 'From' header.
    ///
    /// ```
    /// use brief::mail::{Mailbox, MessageBuilder};
    ///
    /// let sender: Mailbox = "name <user@domain.com>".parse().unwrap();
    /// let builder = MessageBuilder::new().from(sender.into());
    /// ```
    pub fn from(mut self, mailboxes: Mailboxes) -> Self {
        self.headers.push(Header::From(mailboxes));

        self
    }
    /// Convenience method for adding a 'To' header.
    ///
    /// ```
    /// use brief::mail::{Mailbox, MessageBuilder};
    ///
    /// let sender: Mailbox = "name <user@domain.com>".parse().unwrap();
    /// let builder = MessageBuilder::new().to(sender.into());
    /// ```
    pub fn to(mut self, mailboxes: Mailboxes) -> Self {
        self.headers.push(Header::To(mailboxes));

        self
    }
    /// Builds a message using the given headers and body.
    ///
    /// ```
    /// use brief::mail::MessageBuilder;
    ///
    /// let builder = MessageBuilder::new().build();
    /// ```
    pub fn build(self) -> Message {
        Message {
            headers: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::MessageBuilder;

    #[test]
    fn it_creates_a_builder_and_builds_an_empty_message() {
        let message = MessageBuilder::new().build();
        assert_eq!(message.headers.len(), 0);
    }
}
