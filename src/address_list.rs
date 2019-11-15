use std::cmp::PartialEq;
use std::fmt;
use std::iter::{FromIterator, IntoIterator, Iterator};
use std::ops::Deref;

#[cfg(feature="mailparse-conversions")]
use std::convert::TryInto;
#[cfg(feature="mailparse-conversions")]
use super::error::Error;

/// Check if all fields are the same rather than just a subset ("deep equals")
pub trait DeepEq<Rhs = Self> {
    fn deep_eq(&self, other: &Rhs) -> bool;
    fn deep_ne(&self, other: &Rhs) -> bool {
        !self.deep_eq(other)
    }
}

/// Unified interface for all contact types
pub trait Contactish {
    fn email(&self) -> Option<&String>;
    fn name(&self) -> Option<&String>;
    fn comment(&self) -> Option<&String>;
    fn new<T>(required: T) -> Self
    where
        T: AsRef<str>;
    fn set_name<T>(self, name: T) -> Self
    where
        T: AsRef<str>;
    fn set_email<T>(self, email: T) -> Self
    where
        T: AsRef<str>;
    fn set_comment<T>(self, comment: T) -> Self
    where
        T: AsRef<str>;
}

/// For everything that has contacts
pub trait Contactsish {
    fn as_contacts(self) -> Contacts;
}

/// A contact with at least an email address
#[derive(Debug, Clone, Default)]
pub struct EmailContact {
    email: String,
    name: Option<String>,
    comment: Option<String>,
}

impl Contactish for EmailContact {
    fn email(&self) -> Option<&String> {
        Some(&self.email)
    }

    fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    fn comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    fn new<T>(email: T) -> Self
    where
        T: AsRef<str>,
    {
        EmailContact {
            email: email.as_ref().into(),
            name: None,
            comment: None,
        }
    }

    fn set_name<T>(mut self, name: T) -> Self
    where
        T: AsRef<str>,
    {
        let name = name.as_ref().trim();
        if name != "" {
            self.name = Some(name.into());
        }
        self
    }

    fn set_email<T>(mut self, email: T) -> Self
    where
        T: AsRef<str>,
    {
        self.email = email.as_ref().into();
        self
    }

    fn set_comment<T>(mut self, comment: T) -> Self
    where
        T: AsRef<str>,
    {
        let comment = comment.as_ref();
        if comment != "" {
            self.comment = Some(comment.into());
        }
        self
    }
}

/// Check if the email field is the same
impl PartialEq for EmailContact {
    fn eq(&self, other: &EmailContact) -> bool {
        self.email == other.email
    }
}

/// Check if all fields are the same (PartialEq only checks if email is the
/// same)
impl DeepEq for EmailContact {
    fn deep_eq(&self, other: &EmailContact) -> bool {
        self.email == other.email && self.name == other.name && self.comment == other.comment
    }
}

impl fmt::Display for EmailContact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(n) = &self.name {
            write!(f, "\"{}\" ", n.replace("\\", "\\\\").replace("\"", "\\\""))?;
            if let Some(c) = &self.comment {
                write!(f, "({}) ", c)?;
            }
        }
        write!(
            f,
            "<{}>",
            self.email.replace("\\", "\\\\").replace("\"", "\\\""),
        )
    }
}

/// A string that we couldn't parse into an [`EmailContact`] but implements
/// the [`Contactish`] trait regardless
///
/// [`EmailContact`]: struct.EmailContact.html
/// [`Contactish`]: trait.Contactish.html
#[derive(Debug, Clone, Default)]
pub struct GarbageContact(String);

impl Contactish for GarbageContact {
    /// Since we are garbage, we don't have an email address
    fn email(&self) -> Option<&String> {
        None
    }

    /// Since we are garbage, we don't have a name
    fn name(&self) -> Option<&String> {
        None
    }

    /// Returns the actual string we couldn't interpret as [`EmailContact`]
    ///
    /// [`EmailContact`]: struct.EmailContact.html
    fn comment(&self) -> Option<&String> {
        Some(&self.0)
    }

    fn new<T>(garbage: T) -> Self
    where
        T: AsRef<str>,
    {
        GarbageContact(garbage.as_ref().into())
    }

    fn set_comment<T>(mut self, garbage: T) -> Self
    where
        T: AsRef<str>,
    {
        self.0 = garbage.as_ref().into();
        self
    }

    fn set_email<T>(self, _: T) -> Self {
        self
    }

    fn set_name<T>(self, _: T) -> Self {
        self
    }
}

impl From<String> for GarbageContact {
    fn from(string: String) -> Self {
        GarbageContact(string)
    }
}

/// Either an [`EmailContact`] we could successfully parse or a
/// [`GarbageContact`] we didn't want to throw away
///
/// [`EmailContact`]: struct.EmailContact.html
/// [`GarbageContact`]: struct.GarbageContact.html
#[derive(Clone)]
pub enum Contact {
    Email(EmailContact),
    Garbage(GarbageContact),
}

impl Contact {
    pub fn is_garbage(&self) -> bool {
        match self {
            Contact::Garbage(_) => true,
            _ => false,
        }
    }
}

/// Will be handed down on our variants' contents, which implement the same
/// trait
///
/// The exception to the rule is the [`::new`] method.
///
/// **Please note:** the current implementation does not (yet?) magically change
/// a `Contact::Garbage` variant into a `Contact::Email` one if you try to call
/// `::set_email`. It merely returns an unchanged `Self`.
///
/// [`::new`]: enum.Contact.html#method.new
impl Contactish for Contact {
    fn name(&self) -> Option<&String> {
        match self {
            Contact::Email(c) => c.name(),
            Contact::Garbage(_) => None,
        }
    }

    fn email(&self) -> Option<&String> {
        match self {
            Contact::Email(c) => c.email(),
            Contact::Garbage(_) => None,
        }
    }

    fn comment(&self) -> Option<&String> {
        match self {
            Contact::Email(c) => c.comment(),
            Contact::Garbage(c) => c.comment(),
        }
    }

    /// By default we create a new `Contact::Email` variant, since
    /// `Contact::Garbage` is merely a fallback
    fn new<T>(email: T) -> Self
    where
        T: AsRef<str>,
    {
        EmailContact::new(email).into()
    }

    fn set_name<T>(self, name: T) -> Self
    where
        T: AsRef<str>,
    {
        match self {
            Contact::Email(c) => c.set_name(name).into(),
            Contact::Garbage(g) => g.set_name(name).into(),
        }
    }

    fn set_comment<T>(self, comment: T) -> Self
    where
        T: AsRef<str>,
    {
        match self {
            Contact::Email(c) => c.set_comment(comment).into(),
            Contact::Garbage(g) => g.set_comment(comment).into(),
        }
    }

    fn set_email<T>(self, email: T) -> Self
    where
        T: AsRef<str>,
    {
        match self {
            Contact::Email(c) => c.set_email(email).into(),
            Contact::Garbage(g) => g.set_email(email).into(),
        }
    }
}

impl PartialEq for Contact {
    fn eq(&self, other: &Contact) -> bool {
        self.email() == other.email()
    }
}

impl DeepEq for Contact {
    fn deep_eq(&self, other: &Contact) -> bool {
        self.email() == other.email()
            || self.name() == other.name()
            || self.comment() == other.comment()
    }
}

impl fmt::Debug for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Contact({}{}{})",
            match self.name() {
                Some(n) => format!("\"{}\" ", n),
                None => "".into(),
            },
            match self.comment() {
                Some(c) => {
                    if !self.is_garbage() {
                        format!("({}) ", c)
                    } else {
                        format!("Garbage: \"{}\"", c)
                    }
                }
                None => "".into(),
            },
            match self.email() {
                Some(e) => format!("<{}>", e),
                None => "".into(),
            }
        )
    }
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Contact::Garbage(_) => write!(f, ""),
            Contact::Email(e) => write!(f, "{}", e),
        }
    }
}

impl From<GarbageContact> for Contact {
    fn from(garbage: GarbageContact) -> Contact {
        Contact::Garbage(garbage)
    }
}

impl From<EmailContact> for Contact {
    fn from(contact: EmailContact) -> Contact {
        Contact::Email(contact)
    }
}

#[cfg(feature = "mailparse-conversions")]
impl TryInto<mailparse::MailAddr> for Contact {
    type Error = Error;

    fn try_into(self) -> Result<mailparse::MailAddr, Error> {
        match self {
            Contact::Garbage(_) => Err(Error::UnexpectedError(
                "Can't convert Garbage into MailAddr".into(),
            )),
            Contact::Email(_) => Ok(mailparse::MailAddr::Single(self.try_into()?)),
        }
    }
}

#[cfg(feature = "mailparse-conversions")]
impl TryInto<mailparse::SingleInfo> for Contact {
    type Error = Error;

    fn try_into(self) -> Result<mailparse::SingleInfo, Error> {
        match self {
            Contact::Garbage(_) => Err(Error::UnexpectedError(
                "Can't convert Garbage into SingleInfo".into(),
            )),
            Contact::Email(e) => Ok(mailparse::SingleInfo {
                display_name: e.name,
                addr: e.email,
            }),
        }
    }
}

/// Container for [`Contact`]s
///
/// [`Contact`]: enum.Contact.html
///
#[derive(Debug, Clone, Default)]
pub struct Contacts {
    pub contacts: Vec<Contact>,
}

impl Contacts {
    pub fn new() -> Self {
        Self {
            contacts: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.contacts.len()
    }
}

impl Contactsish for Vec<Contact> {
    fn as_contacts(self) -> Contacts {
        Contacts::from(self)
    }
}

impl Contactsish for Contacts {
    fn as_contacts(self) -> Contacts {
        self
    }
}

impl Deref for Contacts {
    type Target = [Contact];

    fn deref<'a>(&'a self) -> &'a [Contact] {
        self.contacts.as_slice()
    }
}

impl<'a> IntoIterator for &'a Contacts {
    type Item = &'a Contact;
    type IntoIter = std::slice::Iter<'a, Contact>;

    fn into_iter(self) -> Self::IntoIter {
        self.contacts.iter()
    }
}

impl IntoIterator for Contacts {
    type Item = Contact;
    type IntoIter = std::vec::IntoIter<Contact>;

    fn into_iter(self) -> Self::IntoIter {
        self.contacts.into_iter()
    }
}

impl FromIterator<Contact> for Contacts {
    fn from_iter<I: IntoIterator<Item = Contact>>(iter: I) -> Contacts {
        let mut contacts = Contacts::new();
        contacts.contacts = Vec::<Contact>::from_iter(iter);
        contacts
    }
}

impl From<Vec<Contact>> for Contacts {
    fn from(s: Vec<Contact>) -> Self {
        Self { contacts: s }
    }
}

impl fmt::Display for Contacts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let trim: &[_] = &[' ', ','];
        write!(
            f,
            "{}",
            self.contacts
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<String>>()
                .join(", ")
                .trim_matches(trim),
        )
    }
}

#[cfg(feature = "mailparse-conversions")]
impl TryInto<Vec<mailparse::SingleInfo>> for Contacts {
    type Error = Error;

    fn try_into(self) -> Result<Vec<mailparse::SingleInfo>, Error> {
        self.into_iter().map(|c| c.try_into()).collect()
    }
}

/// A group with a name and [`Contacts`]
///
/// [`Contacts`]: struct.Contacts.html
#[derive(Debug, Clone, Default)]
pub struct Group {
    pub name: String,
    pub contacts: Contacts,
}

impl Group {
    pub fn new<T>(name: T) -> Self
    where
        T: AsRef<str>,
    {
        let mut new: Self = Default::default();
        new.name = name.as_ref().into();
        new
    }

    pub fn set_contacts<T>(mut self, contacts: T) -> Self
    where
        T: Contactsish,
    {
        self.contacts = contacts.as_contacts();
        self
    }
}

impl PartialEq for Group {
    fn eq(&self, other: &Group) -> bool {
        if self.name != other.name || self.contacts.len() != other.contacts.len() {
            return false;
        }
        for (i, contact) in self.contacts.iter().enumerate() {
            if contact != &other.contacts[i] {
                return false;
            }
        }
        true
    }
}

impl DeepEq for Group {
    fn deep_eq(&self, other: &Group) -> bool {
        if self.name != other.name || self.contacts.len() != other.contacts.len() {
            return false;
        }
        for (i, contact) in self.contacts.iter().enumerate() {
            if !contact.deep_eq(&other.contacts[i]) {
                return false;
            }
        }
        true
    }
}

impl<T> From<T> for Group
where
    T: AsRef<str>,
{
    fn from(string: T) -> Self {
        Self {
            name: string.as_ref().into(),
            contacts: Contacts::new(),
        }
    }
}

#[cfg(feature = "mailparse-conversions")]
impl TryInto<mailparse::MailAddr> for Group {
    type Error = Error;

    fn try_into(self) -> Result<mailparse::MailAddr, Error> {
        Ok(mailparse::MailAddr::Group(mailparse::GroupInfo {
            group_name: self.name,
            addrs: self
                .contacts
                .into_iter()
                .map(|c| c.try_into())
                .collect::<Result<Vec<_>, Error>>()?,
        }))
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\"{}\": {};",
            self.name.replace("\\", "\\\\").replace("\"", "\\\""),
            self.contacts
        )
    }
}

/// All forms which email headers like `To`, `From`, `Cc`, etc. can take
///
/// # Examples
///
/// ```rust
/// # use email_address_list::*;
/// let latvian: AddressList = vec![Contact::new("piemērs@example.org")].into();
/// assert!(latvian.contacts()[0].email().unwrap() == "piemērs@example.org");
///
/// let sudanese: AddressList = Group::new("Conto").into();
/// assert!(sudanese.group_name() == Some(&"Conto".to_string()));
/// ```
#[derive(Debug, Clone)]
pub enum AddressList {
    Contacts(Contacts),
    Group(Group),
}

impl AddressList {
    /// Check if this address list is a group
    pub fn is_group(&self) -> bool {
        match self {
            AddressList::Group(_) => true,
            _ => false,
        }
    }

    /// Get the group name if it is a group
    pub fn group_name(&self) -> Option<&String> {
        match self {
            AddressList::Group(g) => Some(&g.name),
            _ => None,
        }
    }

    /// Get the contacts regardless of our variant
    pub fn contacts(&self) -> &Contacts {
        match self {
            AddressList::Contacts(c) => &c,
            AddressList::Group(g) => &g.contacts,
        }
    }
}

impl fmt::Display for AddressList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressList::Contacts(c) => write!(f, "{}", c),
            AddressList::Group(g) => write!(f, "{}", g),
        }
    }
}

impl PartialEq for AddressList {
    fn eq(&self, other: &AddressList) -> bool {
        if self.is_group() != other.is_group() {
            return false;
        }
        match self {
            AddressList::Group(g) => {
                if let AddressList::Group(o) = other {
                    g == o
                } else {
                    false
                }
            }
            AddressList::Contacts(c) => {
                if let AddressList::Contacts(o) = other {
                    if c.len() != o.len() {
                        return false;
                    }
                    for (i, contact) in c.iter().enumerate() {
                        if contact != &o[i] {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl DeepEq for AddressList {
    fn deep_eq(&self, other: &AddressList) -> bool {
        if self.is_group() != other.is_group() {
            return false;
        }
        match self {
            AddressList::Group(g) => {
                if let AddressList::Group(o) = other {
                    g.deep_eq(&o)
                } else {
                    false
                }
            }
            AddressList::Contacts(c) => {
                if let AddressList::Contacts(o) = other {
                    if c.len() != o.len() {
                        return false;
                    }
                    for (i, contact) in c.iter().enumerate() {
                        if !contact.deep_eq(&o[i]) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl From<Vec<Contact>> for AddressList {
    fn from(s: Vec<Contact>) -> Self {
        Self::Contacts(Contacts { contacts: s })
    }
}

impl Contactsish for AddressList {
    fn as_contacts(self) -> Contacts {
        match self {
            AddressList::Contacts(c) => c,
            AddressList::Group(g) => g.contacts,
        }
    }
}

impl From<Contacts> for AddressList {
    fn from(contacts: Contacts) -> AddressList {
        AddressList::Contacts(contacts)
    }
}

impl From<Group> for AddressList {
    fn from(group: Group) -> AddressList {
        AddressList::Group(group)
    }
}

#[cfg(feature = "mailparse-conversions")]
impl TryInto<Vec<mailparse::MailAddr>> for AddressList {
    type Error = Error;

    fn try_into(self) -> Result<Vec<mailparse::MailAddr>, Error> {
        match self {
            AddressList::Group(g) => Ok(vec![g.try_into()?]),
            AddressList::Contacts(c) => c.into_iter().map(|ic| ic.try_into()).collect(),
        }
    }
}
