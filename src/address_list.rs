#[derive(Debug, Clone)]
pub enum AddressList {
    Contacts(Contacts),
    Group(Group),
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

impl From<Garbage> for Contact {
    fn from(garbage: Garbage) -> Contact {
        Contact::Garbage(garbage)
    }
}

impl From<EmailContact> for Contact {
    fn from(contact: EmailContact) -> Contact {
        Contact::Contact(contact)
    }
}

pub type Garbage = String;
pub type Contacts = Vec<Contact>;

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
pub enum Contact {
    Contact(EmailContact),
    Garbage(Garbage),
}

impl Contact {
    pub fn new_with<T>(email: T, name: Option<T>, comment: Option<T>) -> Self
    where
        T: AsRef<str>,
    {
        Contact::from(EmailContact::new_with(email, name, comment))
    }

    pub fn name(&self) -> Option<String> {
        match self {
            Contact::Contact(c) => c.name.clone(),
            Contact::Garbage(g) => Some(g.clone()),
        }
    }

    pub fn email(&self) -> Option<String> {
        match self {
            Contact::Contact(c) => Some(c.email.clone()),
            Contact::Garbage(_) => None,
        }
    }

    pub fn comment(&self) -> Option<String> {
        match self {
            Contact::Contact(c) => c.comment.clone(),
            Contact::Garbage(_) => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EmailContact {
    pub email: String,
    pub name: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Group {
    pub name: String,
    pub contacts: Contacts,
}

impl Group {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_with<T>(name: T, contacts: Contacts) -> Self
    where
        T: AsRef<str>,
    {
        Group {
            name: name.as_ref().to_string(),
            contacts,
        }
    }
}

impl EmailContact {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_with<T>(
        email: T,
        name: Option<T>,
        comment: Option<T>,
    ) -> EmailContact
    where
        T: AsRef<str>,
    {
        let mut contact = EmailContact::new();
        contact.set_email(email);
        if let Some(n) = name {
            contact.set_name(n);
        }
        if let Some(c) = comment {
            contact.set_comment(c);
        }
        contact
    }

    pub fn set_name<T>(&mut self, name: T)
    where
        T: AsRef<str>,
    {
        let name_r = name.as_ref();
        if name_r.trim() != "" {
            self.name = Some(name_r.trim().to_string());
        }
    }

    pub fn set_email<T>(&mut self, email: T)
    where
        T: AsRef<str>,
    {
        self.email = email.as_ref().to_string();
    }

    pub fn set_comment<T>(&mut self, comment: T)
    where
        T: AsRef<str>,
    {
        let comment_r = comment.as_ref();
        if comment_r != "" {
            self.comment = Some(comment_r.to_string());
        }
    }
}

// XXX this seems supremely dirty, but Vec<Contacts> doesn't allow using
// derive(PartialEq)
impl std::cmp::PartialEq for AddressList {
    fn eq(&self, other: &AddressList) -> bool {
        let contacts: &Contacts;
        let other_contacts: &Contacts;
        let mut groupname: &String = &"".to_string();
        let mut other_groupname: &String = &"".to_string();
        if match self {
            AddressList::Contacts(c) => {
                contacts = c;
                0
            }
            AddressList::Group(g) => {
                contacts = &g.contacts;
                groupname = &g.name;
                1
            }
        } == match other {
            AddressList::Contacts(c) => {
                other_contacts = c;
                0
            }
            AddressList::Group(g) => {
                other_contacts = &g.contacts;
                other_groupname = &g.name;
                1
            }
        } {
            for (i, contact) in contacts.iter().enumerate() {
                if contact != &other_contacts[i] {
                    return false;
                }
            }
            if let AddressList::Group(_) = self {
                if groupname != other_groupname {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

impl DeepEq for AddressList {
    fn deep_eq(&self, other: &AddressList) -> bool {
        let contacts: &Contacts;
        let other_contacts: &Contacts;
        let mut groupname: &String = &"".to_string();
        let mut other_groupname: &String = &"".to_string();
        if match self {
            AddressList::Contacts(c) => {
                contacts = c;
                0
            }
            AddressList::Group(g) => {
                contacts = &g.contacts;
                groupname = &g.name;
                1
            }
        } == match other {
            AddressList::Contacts(c) => {
                other_contacts = c;
                0
            }
            AddressList::Group(g) => {
                other_contacts = &g.contacts;
                other_groupname = &g.name;
                1
            }
        } {
            for (i, contact) in contacts.iter().enumerate() {
                if !contact.deep_eq(&other_contacts[i]) {
                    return false;
                }
            }
            if let AddressList::Group(_) = self {
                if groupname != other_groupname {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

impl std::cmp::PartialEq for EmailContact {
    fn eq(&self, other: &EmailContact) -> bool {
        self.email == other.email
    }
}

impl std::cmp::PartialEq for Contact {
    fn eq(&self, other: &Contact) -> bool {
        self.email() == other.email()
    }
}

pub trait DeepEq<Rhs = Self> {
    fn deep_eq(&self, other: &Rhs) -> bool;
}

impl DeepEq for Contact {
    fn deep_eq(&self, other: &Contact) -> bool {
        self.email() == other.email()
            && self.name() == other.name()
            && self.comment() == other.comment()
    }
}

impl DeepEq for EmailContact {
    fn deep_eq(&self, other: &EmailContact) -> bool {
        self.email == other.email
            && self.name == other.name
            && self.comment == other.comment
    }
}
