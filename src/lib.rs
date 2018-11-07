extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::convert::AsRef;
use pest::Parser;


#[derive(Parser)]
#[grammar="permissive-email-list.pest"]
pub struct ContactListParser;

#[derive(Debug,Clone,Default)]
pub struct Contact {
    pub name: Option<String>,
    pub email: Option<String>,
    pub comment: Option<String>
}

impl Contact {
    pub fn new() -> Self {
        Contact { name: None, email: None, comment: None }
    }

    pub fn new_with<T: AsRef<str>>(name: Option<T>, email: Option<T>,
                    comment: Option<T>) ->  Contact {
        Contact {
            name: match name {
                Some(n) => Some(n.as_ref().to_string()),
                None => None
            },
            email: match email {
                Some(e) => Some(e.as_ref().to_string()),
                None => None
            },
            comment: match comment {
                Some(c) => Some(c.as_ref().to_string()),
                None => None
            }
        }
    }

    pub fn set_name<T: AsRef<str>>(&mut self, name: T) {
        let name_r = name.as_ref();
        if name_r != "" {
            self.name = Some(name_r.trim().to_string());
        }
    }

    pub fn set_email<T: AsRef<str>>(&mut self, email: T) {
        self.email = Some(email.as_ref().to_string());
    }

    pub fn set_comment<T: AsRef<str>>(&mut self, comment: T) {
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

trait DeepEq<Rhs = Self> {
    fn deep_eq(&self, other: &Rhs) -> bool;
}

impl DeepEq for Contact {
    fn deep_eq(&self, other: &Contact) -> bool {
        self.email == other.email &&
        self.name == other.name &&
        self.comment == other.comment
    }
}

pub fn parse_contact_list<T: AsRef<str>>(contact_list: Option<T>) ->
       Vec<Contact> {
    let mut contacts = Vec::<Contact>::new();

    let contact_list_u: T;
    let contact_list_r: &str;

    match contact_list {
        Some(c) => {
            contact_list_u = c;
            contact_list_r = contact_list_u.as_ref();
            if contact_list_r == "" {
                return contacts;
            }
        }
        None => return contacts
    }

    let pairs = ContactListParser::parse(Rule::all, contact_list_r);
    for pair in pairs.unwrap().flatten() {
        let mut ct = Contact::new();
        match pair.as_rule() {
            Rule::contact => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::name => ct.set_name(inner.into_inner()
                                                       .next()
                                                       .unwrap()
                                                       .as_str()),
                        Rule::email => ct.set_email(inner.as_str()),
                        Rule::email_angle => ct.set_email(inner.into_inner()
                                                               .next()
                                                               .unwrap()
                                                               .as_str()),
                        Rule::comment => ct.set_comment(inner.as_str()),
                        Rule::malformed => ct.set_name(inner.as_str()),
                        _ => {}
                    }
                }
            },
            Rule::group => {
                ct.set_name(pair.into_inner()
                                .next()
                                .unwrap()
                                .into_inner()
                                .next()
                                .unwrap()
                                .as_str());
            },
            // Anything that we can't turn into an actual address even by
            // rather permissive parsing, make into a Contact that only has
            // a name (i.e. a contact that can't be replied to without a
            // Reply-To header)
            Rule::garbage => {
                ct.set_name(pair.as_str());
            },
            _ => {}
        }
        if ct.any_some() {
            contacts.push(ct);
        }
    }
    contacts
}

#[cfg(test)]
mod tests;
