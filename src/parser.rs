use pest::iterators::{Pair, Pairs};
use pest::Parser as PestParser;

use error::Error::*;
use error::*;

use std::convert::AsRef;

use address_list::*;

#[derive(Parser)]
#[grammar = "../grammars/permissive.pest"]
struct Parser;

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
                return Ok(GarbageContact::new(inner.as_str()).into());
            }
            _ => return Err(invalid_nesting("contact")),
        }
    }
    Ok(c.into())
}

fn parse_pairs(pairs: Pairs<Rule>) -> Result<AddressList> {
    let mut contacts = Contacts::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::contact => {
                match parse_contact_pair(pair) {
                    // Only add GarbageContent that isn't empty
                    Ok(Contact::Garbage(g)) => {
                        // GarbageContacts::comment() will always return Some,
                        // so the unwrap here is unproblematic
                        if g.comment().unwrap() != "" {
                            contacts.push(g.into());
                        }
                    },
                    Ok(c) => contacts.push(c),
                    Err(e) => return Err(e),
                }
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
            Rule::address_list => return parse_pairs(pair.into_inner()),
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

fn check_empty<T>(address_list: &T) -> Result<()>
where
    T: AsRef<str>,
    T: ?Sized,
{
    match address_list.as_ref().trim() {
        "" => Err(Error::Empty),
        _ => Ok(()),
    }
}

/// Get an [`AddressList`] from a string
///
/// Tries its best to come up with the most reasonable parsed address list for a
/// given (potentially spec-violating) input.
///
/// If there's nothing to parse (i.e. an empty string), this function "fails"
/// with [`Error::Empty`], which is essentially equivalent to a `None`, but
/// avoids nesting types.
///
/// # Examples
///
/// Named malformed group:
///
/// ```rust
/// # use email_address_list::*;
/// let input = r#"Kikundi:  ,  "Jina"  (Maoni) <jina@example.org>, baruapepe@example.org;"#;
///
/// let result = parse_address_list(input).unwrap();
///
/// let manual: AddressList = Group::new("Kikundi").set_contacts(vec![
///     Contact::new("jina@example.org").set_name("Jina").set_comment("Maoni"),
///     Contact::new("baruapepe@example.org")
/// ]).into();
///
/// assert!(result.deep_eq(&manual));
/// ```
///
/// Multiple contacts, some of which may be malformed:
///
/// ```rust
/// # use email_address_list::*;
/// let input = r#"Przykład <przykład@example.org>, Példa, Rosszformázott <példa@example.org>"#;
///
/// let manual: AddressList = vec![
///     Contact::new("przykład@example.org").set_name("Przykład"),
///     Contact::new("példa@example.org").set_name("Példa, Rosszformázott"),
/// ].into();
///
/// println!("{:?}", manual);
///
/// let result = parse_address_list(input).unwrap();
///
/// assert!(result.deep_eq(&manual));
/// ```
///
/// Supplying an empty string:
///
/// ```rust
/// # use email_address_list::*;
/// match parse_address_list("") {
///     Err(error::Error::Empty) => assert!(true),
///     Ok(_) | Err(_) => assert!(false),
/// };
/// ```
///
/// [`AddressList`]: enum.AddressList.html
/// [`Error::Empty`]: error/enum.Error.html
pub fn parse_address_list<T>(address_list: &T) -> Result<AddressList>
where
    T: AsRef<str>,
    T: ?Sized,
{
    check_empty(address_list)?;
    Ok(parse_pairs(Parser::parse(
        Rule::address_list,
        address_list.as_ref().trim(),
    )?)?)
}

/// Parse only a single [`Contact`], ignore the rest
///
/// Just like [`parse_address_list`], this function "fails" with
/// [`Error::Empty`] when the supplied string is empty.
///
/// # Examples
///
/// Single contact:
///
/// ```rust
/// # use email_address_list::*;
/// let single = parse_contact("<retpoŝto+kontakto@example.org>").unwrap();
///
/// assert!(single.deep_eq(&Contact::new("retpoŝto+kontakto@example.org")));
/// ```
///
/// Multiple contacts:
///
/// ```rust
/// # use email_address_list::*;
/// let multiple = parse_contact("courriel@example.org, exemple@example.org").unwrap();
///
/// assert!(multiple.deep_eq(&Contact::new("courriel@example.org")));
/// ```
///
/// Not a contact:
///
/// ```rust
/// # use email_address_list::*;
/// match parse_contact("Mist").unwrap() {
///     Contact::Garbage(_) => assert!(true),
///     Contact::Email(_) => assert!(false),
/// }
/// ```
///
/// Empty input:
///
/// ```rust
/// # use email_address_list::*;
/// match parse_contact("") {
///     Err(error::Error::Empty) => assert!(true),
///     Ok(_) | Err(_) => assert!(false),
/// }
/// ```
///
/// [`Contact`]: enum.Contact.html
/// [`parse_address_list`]: fn.parse_address_list.html
/// [`Error::Empty`]: error/enum.Error.html
pub fn parse_contact<T>(contact: &T) -> Result<Contact>
where
    T: AsRef<str>,
    T: ?Sized,
{
    check_empty(contact)?;
    Ok(parse_contact_pair(match Parser::parse(
        Rule::contact,
        contact.as_ref().trim(),
    )?.next()
    {
        Some(c) => c,
        None => return Err(Error::Empty),
    })?)
}