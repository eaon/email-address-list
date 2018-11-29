/*!
Näive [Pest](https://pest.rs/) based parser, picking out "contacts" from email
address lists found in headers such as `from`, `to`, `cc`, etc.

This library aims to be practical rather than "correct". It is (potentially
excessively) permissive to parse even the worst garbage in anyone's inbox.
Limited testing with real world data has been, but the grammar that forms the
basis for this library probably still needs work to catch more edge cases.

# Example

```rust
use email_address_list::*;
# fn main() -> error::Result<()> {

let manual = AddressList::from(vec![
    Contact::new("flastname@example.org").set_name("Firstname Lastname")
]);

let result = parse_address_list(&Some("Firstname Lastname <flastname@example.org>"))?;

assert_eq!(result, manual);
# Ok(())
# }
```
*/
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::convert::AsRef;
use std::convert::From;

#[derive(Parser)]
#[grammar = "../grammars/permissive.pest"]
pub struct AddressListParser;

pub mod error;
use error::Error::*;
use error::*;

mod address_list;
pub use address_list::*;

/// Anything that we can't turn into an actual address even by rather permissive
/// parsing, make into a Contact that only has a name (i.e. a contact that can't
/// be replied to without a Reply-To header)
pub fn parse_address_list<T>(address_list: &Option<T>) -> Result<AddressList>
where
    T: AsRef<str>,
{
    let address_list = check_empty(address_list)?;
    Ok(parse_pairs(AddressListParser::parse(
        Rule::all,
        address_list.as_ref(),
    )?)?)
}

pub fn parse_contact<T>(contact: &Option<T>) -> Result<Contact>
where
    T: AsRef<str>,
{
    let contact = check_empty(contact)?;
    Ok(parse_contact_pair(match AddressListParser::parse(
        Rule::contact,
        contact.as_ref(),
    )?.next()
    {
        Some(c) => c,
        None => return Err(Error::Empty),
    })?)
}

fn parse_contact_pair(pair: Pair<Rule>) -> Result<Contact> {
    let mut c: EmailContact = Default::default();
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::malformed => c = c.set_name(inner.as_str()),
            Rule::name => match inner.into_inner().next() {
                Some(s) => c = c.set_name(s.as_str()),
                None => return Err(invalid_empty("name")),
            },
            Rule::email | Rule::mailbox => c = c.set_email(inner.as_str()),
            Rule::email_angle | Rule::mailbox_angle => match inner
                .into_inner()
                .next()
            {
                Some(s) => c = c.set_email(s.as_str()),
                None => {
                    return Err(invalid_empty("email_angle or mailbox_angle"));
                }
            },
            Rule::comment => c = c.set_comment(inner.as_str()),
            Rule::garbage => {
                return Ok(GarbageContact::from(inner.as_str()).into());
            }
            _ => return Err(invalid_nesting("contact")),
        }
    }
    Ok(Contact::from(c))
}

fn check_empty<T>(address_list: &Option<T>) -> Result<&T>
where
    T: AsRef<str>,
{
    match address_list {
        Some(c) => match c.as_ref().trim() {
            "" => Err(Error::Empty),
            _ => Ok(c),
        },
        None => Err(Error::Empty),
    }
}

fn parse_pairs(pairs: Pairs<Rule>) -> Result<AddressList> {
    let mut contacts = Contacts::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::contact => {
                contacts.push(parse_contact_pair(pair)?);
            }
            Rule::group => {
                let mut group: Group = Default::default();
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
