/*!
[Pest based parser](https://pest.rs/) picking out "contacts" from email address
lists found in headers such as `from`, `to`, `cc`, etc.

This library aims to be practical rather than "correct". It is (potentially
excessively) permissive to parse even the worst garbage in anyone's inbox.
Limited testing with real world data has been, but the grammar that forms the
basis for this library probably still needs work to catch more edge cases.
*/
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pairs;
use pest::Parser;
use std::convert::AsRef;
use std::convert::From;

#[derive(Parser)]
#[grammar = "../grammars/permissive.pest"]
pub struct AddressListParser;

#[derive(Debug, Clone)]
pub enum AddressList {
    Contacts(Vec<Contact>),
    Group(Group),
    Garbage(Garbage),
}

impl From<Vec<Contact>> for AddressList {
    fn from(contacts: Vec<Contact>) -> AddressList {
        AddressList::Contacts(contacts)
    }
}

impl From<Group> for AddressList {
    fn from(group: Group) -> AddressList {
        AddressList::Group(group)
    }
}

impl From<Garbage> for AddressList {
    fn from(garbage: Garbage) -> AddressList {
        AddressList::Garbage(garbage)
    }
}

impl<T> From<T> for Group
where
    T: AsRef<str>,
{
    fn from(string: T) -> Group {
        Group {
            name: string.as_ref().to_string(),
            contacts: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Contact {
    pub name: Option<String>,
    /// If `email` is `None`, we either couldn't parse anything at all, or we
    /// parsed the name of a group.
    pub email: String,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Group {
    pub name: String,
    pub contacts: Vec<Contact>,
}

impl Group {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_with<T>(name: T, contacts: Vec<Contact>) -> Self
    where
        T: AsRef<str>,
    {
        Group {
            name: name.as_ref().to_string(),
            contacts
        }
    }
}

pub type Garbage = String;

#[derive(Debug)]
pub enum Error {
    PestRuleError(pest::error::Error<Rule>),
    UnspecifiedError(String),
}

impl std::convert::From<pest::error::Error<Rule>> for Error {
    fn from(s: pest::error::Error<Rule>) -> Error {
        Error::PestRuleError(s)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl Contact {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_with<T>(email: T, name: Option<T>, comment: Option<T>) -> Contact
    where
        T: AsRef<str>,
    {
        let mut contact = Contact::new();
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
        if name_r != "" {
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

impl std::cmp::PartialEq for Contact {
    fn eq(&self, other: &Contact) -> bool {
        self.email == other.email
    }
}

pub trait DeepEq<Rhs = Self> {
    fn deep_eq(&self, other: &Rhs) -> bool;
}

impl DeepEq for Contact {
    fn deep_eq(&self, other: &Contact) -> bool {
        self.email == other.email
            && self.name == other.name
            && self.comment == other.comment
    }
}

impl DeepEq for AddressList {
    fn deep_eq(&self, other: &AddressList) -> bool {
        true
    }
}

/// Anything that we can't turn into an actual address even by rather permissive
/// parsing, make into a Contact that only has a name (i.e. a contact that can't
/// be replied to without a Reply-To header)
pub fn parse_address_list<T>(
    contact_list: &Option<T>,
) -> Result<Option<AddressList>>
where
    T: AsRef<str>,
{
    Ok(Some(parse_pairs(AddressListParser::parse(
        Rule::all,
        match contact_list {
            Some(ref c) => match c.as_ref() {
                "" => return Ok(None),
                s => s,
            },
            None => return Ok(None),
        },
    )?)?))
}

pub fn parse_pairs(pairs: Pairs<Rule>) -> Result<AddressList> {
    let mut contacts = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::contact => {
                let mut c = Contact::new();
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::malformed => c.set_name(inner.as_str()),
                        Rule::name => match inner.into_inner().next() {
                            Some(s) => c.set_name(s.as_str()),
                            None => return Err(Error::UnspecifiedError("set_name".to_string())),
                        },
                        Rule::email => c.set_email(inner.as_str()),
                        Rule::email_angle => match inner.into_inner().next() {
                            Some(s) => c.set_email(s.as_str()),
                            None => return Err(Error::UnspecifiedError("set_email".to_string())),
                        },
                        Rule::comment => c.set_comment(inner.as_str()),
                        _ => return Err(Error::UnspecifiedError("set_comment".to_string())),
                    }
                }
                contacts.push(c);
            }
            Rule::group => {
                let mut group = Group::new();
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::name => {
                            group.name = inner.as_str().to_string();
                        }
                        Rule::contact_list => {
                            group.contacts = match parse_pairs(inner.into_inner())? {
                                AddressList::Contacts(c) => c,
                                _ => return Err(Error::UnspecifiedError("Unexpected Group Pairs".to_string())),
                            };
                        }
                        _ => return Err(Error::UnspecifiedError("Unexpected Group Pair".to_string())),
                    }
                }
                return Ok(AddressList::from(group));
            }
            Rule::garbage => {
                return Ok(AddressList::from(Garbage::from(pair.as_str())));
            }
            Rule::all => return parse_pairs(pair.into_inner()),
            Rule::contact_list => return parse_pairs(pair.into_inner()),
            _ => return Err(Error::UnspecifiedError(format!("_: {:#?}", pair))),
        }
    }
    Ok(AddressList::from(contacts))
}

#[cfg(test)]
mod tests;
