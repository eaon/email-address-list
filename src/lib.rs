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

mod error;
use error::Error::*;
pub use error::*;

mod address_list;
pub use address_list::*;

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
            Some(ref c) => match c.as_ref().trim() {
                "" => return Ok(None),
                s => s,
            },
            None => return Ok(None),
        },
    )?)?))
}

fn parse_pairs(pairs: Pairs<Rule>) -> Result<AddressList> {
    fn invalid_nesting(rule: &str) -> Error {
        UnexpectedError(format!("Invalid nesting in {} rule", rule))
    }

    fn invalid_empty(rule: &str) -> Error {
        UnexpectedError(format!("{} cannot be empty", rule))
    }

    let mut contacts = Contacts::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::contact => {
                let mut c = EmailContact::new();
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::malformed => c.set_name(inner.as_str()),
                        Rule::name => match inner.into_inner().next() {
                            Some(s) => c.set_name(s.as_str()),
                            None => return Err(invalid_empty("name")),
                        },
                        Rule::email | Rule::mailbox => {
                            c.set_email(inner.as_str())
                        }
                        Rule::email_angle | Rule::mailbox_angle => {
                            match inner.into_inner().next() {
                                Some(s) => c.set_email(s.as_str()),
                                None => {
                                    return Err(invalid_empty(
                                        "email_angle or mailbox_angle",
                                    ));
                                }
                            }
                        }
                        Rule::comment => c.set_comment(inner.as_str()),
                        Rule::garbage => {
                            return Ok(AddressList::from(vec![Contact::from(
                                GarbageContact::from(inner.as_str()),
                            )]));
                        }
                        _ => {
                            return Err(invalid_nesting("contact"));
                        }
                    }
                }
                contacts.push(Contact::from(c));
            }
            Rule::group => {
                let mut group = Group::new();
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::name => {
                            group.name =
                                inner.into_inner().as_str().to_string();
                        }
                        Rule::contact_list => {
                            group.contacts =
                                match parse_pairs(inner.into_inner())? {
                                    AddressList::Contacts(c) => c,
                                    _ => return Err(invalid_nesting("group")),
                                };
                        }
                        _ => return Err(invalid_nesting("group")),
                    }
                }
                return Ok(AddressList::from(group));
            }
            Rule::all => return parse_pairs(pair.into_inner()),
            Rule::contact_list => return parse_pairs(pair.into_inner()),
            _ => {
                return Err(UnexpectedError(format!(
                    "{:?} can't be parsed with this function",
                    pair.as_rule(),
                )));
            }
        }
    }
    Ok(AddressList::from(contacts))
}

#[cfg(test)]
mod tests;
