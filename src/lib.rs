extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::convert::AsRef;

#[derive(Parser)]
#[grammar = "../grammars/permissive.pest"]
pub struct AddressListParser;

#[derive(Debug, Clone, Default)]
pub struct Contact {
    pub name: Option<String>,
    pub email: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug)]
pub enum Error {
    PestRuleError(pest::error::Error<Rule>),
    UnspecifiedError,
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

    pub fn new_with<T>(
        name: Option<T>,
        email: Option<T>,
        comment: Option<T>,
    ) -> Contact
    where
        T: AsRef<str>,
    {
        let mut contact = Contact::new();
        if let Some(n) = name {
            contact.set_name(n);
        }
        if let Some(e) = email {
            contact.set_email(e);
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
        self.email = Some(email.as_ref().to_string());
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

    pub fn any_some(&self) -> bool {
        self.name.is_some() || self.email.is_some()
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

pub fn parse_contact_list<T>(contact_list: &Option<T>) -> Result<Vec<Contact>>
where
    T: AsRef<str>,
{
    let mut contacts = Vec::<Contact>::new();

    let contact_list = match contact_list {
        Some(ref c) => {
            let cl = c.as_ref();
            if cl == "" {
                return Ok(contacts);
            }
            cl
        }
        None => return Ok(contacts),
    };

    let pairs = AddressListParser::parse(Rule::all, contact_list);
    for pair in pairs?.flatten() {
        let mut ct = Contact::new();
        match pair.as_rule() {
            Rule::contact => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::name => match inner.into_inner().next() {
                            Some(s) => ct.set_name(s.as_str()),
                            None => return Err(Error::UnspecifiedError),
                        },
                        Rule::email => ct.set_email(inner.as_str()),
                        Rule::email_angle => match inner.into_inner().next() {
                            Some(s) => ct.set_email(s.as_str()),
                            None => return Err(Error::UnspecifiedError),
                        },
                        Rule::comment => ct.set_comment(inner.as_str()),
                        Rule::malformed => ct.set_name(inner.as_str()),
                        _ => {}
                    }
                }
            }
            Rule::group => {
                let inner = match pair.into_inner().next() {
                    Some(i) => i,
                    None => return Err(Error::UnspecifiedError),
                };
                match inner.into_inner().next() {
                    Some(s) => ct.set_name(s.as_str()),
                    None => return Err(Error::UnspecifiedError),
                }
            }
            // Anything that we can't turn into an actual address even by
            // rather permissive parsing, make into a Contact that only has
            // a name (i.e. a contact that can't be replied to without a
            // Reply-To header)
            Rule::garbage => {
                ct.set_name(pair.as_str());
            }
            _ => {}
        }
        if ct.any_some() {
            contacts.push(ct);
        }
    }
    Ok(contacts)
}

#[cfg(test)]
mod tests;
