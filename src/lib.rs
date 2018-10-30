extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;


#[derive(Parser)]
#[grammar="permissive-email-list.pest"]
pub struct ContactListParser;

#[derive(Debug,Clone)]
pub struct Contact {
    name: Option<String>,
    email: Option<String>
}

impl Contact {
    pub fn new() -> Contact {
        Contact { name: None, email: None }
    }

    pub fn new_with(name: Option<&str>, email: Option<&str>) -> Contact {
        Contact {
            name: match name {
                Some(n) => Some(n.to_string()),
                None => None
            },
            email: match email {
                Some(e) => Some(e.to_string()),
                None => None
            }
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.trim().to_string());
    }

    pub fn set_email(&mut self, email: &str) {
        self.email = Some(email.to_string());
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
        self.name == other.name
    }
}

pub fn parse_contact_list(contact_list: &str) -> Vec<Contact> {
    let mut contacts = Vec::<Contact>::new();

    if contact_list == "" {
        return contacts;
    }

    let pairs = ContactListParser::parse(Rule::all, contact_list);
    for pair in pairs.unwrap().flatten() {
        let mut ct = Contact::new();
        match pair.as_rule() {
            Rule::contact => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::name => ct.set_name(inner.into_inner()
                                                       .next()
                                                       .unwrap().as_str()),
                        Rule::email => ct.set_email(inner.as_str()),
                        Rule::email_angle => ct.set_email(inner.into_inner()
                                                               .next()
                                                               .unwrap()
                                                               .as_str()),
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
                println!("{}", pair.as_str());
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
