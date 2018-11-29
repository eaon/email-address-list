use std::cmp::PartialEq;
use std::fmt;

/// "Deep Equals" will check if all fields are the same
pub trait DeepEq<Rhs = Self> {
    fn deep_eq(&self, other: &Rhs) -> bool;
    fn deep_ne(&self, other: &Rhs) -> bool {
        !self.deep_eq(other)
    }
}

/// Provides a unified interface for all contact relevant bits of information
pub trait ContactInfo {
    fn email(&self) -> Option<&String>;
    fn name(&self) -> Option<&String>;
    fn comment(&self) -> Option<&String>;
}

/// Unified interface for all contact types that
pub trait ContactMutable {
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

#[derive(Debug, Clone, Default)]
pub struct EmailContact {
    email: String,
    name: Option<String>,
    comment: Option<String>,
}

impl ContactInfo for EmailContact {
    fn email(&self) -> Option<&String> {
        Some(&self.email)
    }

    fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    fn comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }
}

impl ContactMutable for EmailContact {
    fn new<T>(email: T) -> Self
    where
        T: AsRef<str>,
    {
        EmailContact {
            email: email.as_ref().to_string(),
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
            self.name = Some(name.to_string());
        }
        self
    }

    fn set_email<T>(mut self, email: T) -> Self
    where
        T: AsRef<str>,
    {
        self.email = email.as_ref().to_string();
        self
    }

    fn set_comment<T>(mut self, comment: T) -> Self
    where
        T: AsRef<str>,
    {
        let comment = comment.as_ref();
        if comment != "" {
            self.comment = Some(comment.to_string());
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
        self.email == other.email
            && self.name == other.name
            && self.comment == other.comment
    }
}

pub type GarbageContact = String;

impl ContactInfo for GarbageContact {
    fn email(&self) -> Option<&String> {
        None
    }

    fn name(&self) -> Option<&String> {
        Some(&self)
    }

    fn comment(&self) -> Option<&String> {
        None
    }
}

impl ContactMutable for GarbageContact {
    fn new<T>(garbage: T) -> Self
    where
        T: AsRef<str>,
    {
        garbage.as_ref().to_string()
    }

    fn set_name<T>(self, garbage: T) -> Self
    where
        T: AsRef<str>,
    {
        garbage.as_ref().to_string()
    }

    fn set_email<T>(self, _: T) -> Self {
        self
    }

    fn set_comment<T>(self, _: T) -> Self {
        self
    }
}

/// Contact::from("Random bits".to_string())
#[derive(Clone)]
pub enum Contact {
    Contact(EmailContact),
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

impl ContactInfo for Contact {
    fn name(&self) -> Option<&String> {
        match self {
            Contact::Contact(c) => c.name(),
            Contact::Garbage(g) => g.name(),
        }
    }

    fn email(&self) -> Option<&String> {
        match self {
            Contact::Contact(c) => c.email(),
            Contact::Garbage(_) => None,
        }
    }

    fn comment(&self) -> Option<&String> {
        match self {
            Contact::Contact(c) => c.comment(),
            Contact::Garbage(_) => None,
        }
    }
}

impl ContactMutable for Contact {
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
            Contact::Contact(c) => c.set_name(name).into(),
            Contact::Garbage(g) => g.set_name(name).into(),
        }
    }

    fn set_comment<T>(self, comment: T) -> Self
    where
        T: AsRef<str>,
    {
        match self {
            Contact::Contact(c) => c.set_comment(comment).into(),
            Contact::Garbage(g) => g.set_comment(comment).into(),
        }
    }

    fn set_email<T>(self, email: T) -> Self
    where
        T: AsRef<str>,
    {
        match self {
            Contact::Contact(c) => c.set_email(email).into(),
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Contact({}{}{})",
            match self.name() {
                Some(n) => format!("\"{}\" ", n),
                None => "".to_string(),
            },
            match self.comment() {
                Some(c) => format!("({}) ", c),
                None => "".to_string(),
            },
            match self.email() {
                Some(e) => format!("<{}>", e),
                None => "".to_string(),
            }
        )
    }
}

impl From<GarbageContact> for Contact {
    fn from(garbage: GarbageContact) -> Contact {
        Contact::Garbage(garbage)
    }
}

impl From<EmailContact> for Contact {
    fn from(contact: EmailContact) -> Contact {
        Contact::Contact(contact)
    }
}

pub type Contacts = Vec<Contact>;

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
        new.name = name.as_ref().to_string();
        new
    }

    pub fn set_contacts(mut self, contacts: Contacts) -> Self {
        self.contacts = contacts;
        self
    }
}

impl ContactInfo for Group {
    fn email(&self) -> Option<&String> {
        None
    }

    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn comment(&self) -> Option<&String> {
        None
    }
}

impl PartialEq for Group {
    fn eq(&self, other: &Group) -> bool {
        if self.name != other.name
            || self.contacts.len() != other.contacts.len()
        {
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
        if self.name != other.name
            || self.contacts.len() != other.contacts.len()
        {
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
    fn from(string: T) -> Group {
        Group {
            name: string.as_ref().to_string(),
            contacts: Contacts::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AddressList {
    Contacts(Contacts),
    Group(Group),
}

impl AddressList {
    pub fn is_group(&self) -> bool {
        match self {
            AddressList::Group(_) => true,
            _ => false,
        }
    }

    pub fn group_name(&self) -> Option<&String> {
        match self {
            AddressList::Group(g) => Some(&g.name),
            _ => None,
        }
    }

    pub fn contacts(&self) -> &Contacts {
        match self {
            AddressList::Contacts(c) => &c,
            AddressList::Group(g) => &g.contacts,
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
                    return g == o;
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
                }
            }
        }
        true
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
                    return g.deep_eq(&o);
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
                }
            }
        }
        true
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
