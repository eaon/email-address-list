use lazy_static::*;
use pest::iterators::{Pair, Pairs};
use pest::Parser as PestParser;
use pest_derive::Parser;
use regex::Regex;

use crate::error::Error::*;
use crate::error::*;

use std::convert::AsRef;

use crate::address_list::*;

lazy_static! {
    static ref CSV: Regex = Regex::new(
        r#"[^",]*"[^"\\]*\\.[^"\\]+"[^,"]+@[^,"]+|[^",]*".*?"[^,"]*@[^,"]*|[^,"]+@[^,"]+"#,
    )
    .unwrap();
    static ref SSV: Regex = Regex::new(r#"[^;"]?".*?"[^;"]*|[^;"]*"#).unwrap();
}

#[derive(Parser)]
#[grammar = "../grammars/permissive.pest"]
struct Parser;

fn parse_contact_pair(pair: Pair<'_, Rule>) -> Option<Result<Contact>> {
    let mut c: EmailContact = Default::default();
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::malformed | Rule::malformed_comment_name => c = c.set_name(inner.as_str()),
            Rule::name => match inner.into_inner().next() {
                Some(s) => c = c.set_name(s.as_str()),
                None => return Some(Err(invalid_empty("name"))),
            },
            Rule::email | Rule::mailbox => c = c.set_email(inner.as_str()),
            Rule::email_angle | Rule::mailbox_angle => match inner.into_inner().next() {
                Some(s) => c = c.set_email(s.as_str()),
                None => {
                    return Some(Err(invalid_empty("email_angle or mailbox_angle")));
                }
            },
            Rule::comment => c = c.set_comment(inner.as_str()),
            Rule::garbage => {
                let garbage = inner.as_str();
                if garbage == "" {
                    return None;
                }
                return Some(Ok(GarbageContact::new(garbage).into()));
            }
            Rule::garbage_nongreedy => {
                let garbage = inner.as_str().trim();
                // garbage_nongreedy is special in the sense that we know that a mailbox
                // precedes it - the only occurance of this I've seen was when domain names were
                // separated by whitespace
                let new_email = format!("{}{}", c.email().unwrap(), garbage);
                c = c.set_email(new_email);
            }
            _ => return Some(Err(invalid_nesting("contact"))),
        }
    }
    Some(Ok(c.into()))
}

fn parse_pairs(pairs: Pairs<'_, Rule>) -> Result<AddressList> {
    let mut contacts = Contacts::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::group => {
                let mut group: Group = Default::default();
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::name => {
                            group.name = inner.into_inner().as_str().to_string();
                        }
                        Rule::contact_list => {
                            group.contacts = inner
                                .into_inner()
                                .filter_map(parse_contact_pair)
                                .collect::<Result<Contacts>>()?
                        }
                        _ => return Err(invalid_nesting("group")),
                    }
                }
                return Ok(AddressList::from(group));
            }
            Rule::address_list => return parse_pairs(pair.into_inner()),
            Rule::contact_list => {
                contacts = pair
                    .into_inner()
                    .filter_map(parse_contact_pair)
                    .collect::<Result<Contacts>>()?
            }
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

fn check_empty<T>(address_list: &T) -> Result<&str>
where
    T: AsRef<str>,
    T: ?Sized,
{
    let input = address_list.as_ref().trim();
    match input {
        "" => Err(Error::Empty),
        _ => Ok(input),
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
    let input = check_empty(address_list)?;
    let mut output = parse_pairs(Parser::parse(Rule::address_list, input)?)?;

    /// Make estimation of correct parsing easier
    ///
    /// Remove all common characters, the lengths of this output should be roughly equal for what
    /// we put in and what we plan to put out.
    fn normalise(input: &str) -> String {
        (&[",", "\"", "'", "<", ">"]
            .iter()
            .fold(input.to_string(), |o, p| o.replace(p, "")))
            .replace(char::is_whitespace, "")
    }

    /// Comma separated values optimised for the way they are used in address lists
    fn csv(input: &str) -> Vec<String> {
        CSV.captures_iter(input)
            .filter_map(|c| {
                if let Some(c) = c.get(0) {
                    return Some(c.as_str().into());
                }
                None
            })
            .collect()
    }

    /// Break apart undelimited addresses if they are present and put them in the appropriate place
    /// of the list
    // XXX add a way to fish out both addresses from something like:
    // one@example.org Firstname Surname <two@example.org>
    fn expand_undelimited(mut input: Vec<String>) -> Vec<String> {
        let mut output = <Vec<String>>::new();
        let mut r = 0;
        for i in 0..input.len() {
            let j = &input[i - r];
            if j.contains('>') {
                for s in j.split('>') {
                    if s.contains('<') {
                        output.push(format!("{}>", s));
                    } else if s == "" {
                        // don't do anything with empty bits
                    } else {
                        output.push(s.into());
                    }
                }
                input.remove(i - r);
            } else {
                output.push(input.remove(i - r));
            }
            r += 1;
        }
        output
    }

    fn add_absent_contacts(input: &[String], output: &mut AddressList) -> Result<()> {
        for contact in input.iter().map(|c| parse_contact(c)) {
            let contact = contact?;
            if let Contact::Email(_) = contact {
                if !output.contains(&contact) {
                    output.add(contact);
                }
            }
        }
        Ok(())
    }

    let input_n = normalise(input);
    let output_n = normalise(&format!("{}", output));

    if input_n.len() > output_n.len() {
        let input_c = csv(input);
        // Due to the way some headers are malformed, the grammar cannot account for all ways in
        // which data out there is separated, This check is for an educated guess about
        // whether we have a ';' separated address list, and returns it if necessary
        if let AddressList::Contacts(_) = output {
            if input_n.contains(';') {
                let sc_input = SSV.captures_iter(input).fold(String::from(""), |mut f, c| {
                    if let Some(cpt) = c.get(0) {
                        f.push_str(cpt.as_str());
                        f.push(',');
                    }
                    f
                });
                let mut sc_output = parse_pairs(Parser::parse(
                    Rule::address_list,
                    &sc_input.trim_end_matches(','),
                )?)?;
                // If the semi-colon delimited output is bigger than the regular one we're likely
                // a completely semi-colon separated list, however, we're still trying to find
                // as many contacts as possible by looking for undelimited ones
                if sc_output.len() > output.len() && sc_output.len() > input_c.len() {
                    let sc_output_n = normalise(&format!("{}", sc_output));
                    if input_n.len() > sc_output_n.len() {
                        let sc_input_c_a = expand_undelimited(csv(&sc_input));
                        add_absent_contacts(&sc_input_c_a, &mut sc_output)?;
                    }
                    return Ok(sc_output);
                }
            }
        }

        // Last resort, deal with split commas as individual contacts and build an AddressList from
        // that
        let input_c_a = expand_undelimited(input_c);
        if input_c_a.len() > output.len() {
            add_absent_contacts(&input_c_a, &mut output)?;
        }
    }
    Ok(output)
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
/// match parse_contact(",") {
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
    let contact = check_empty(contact)?;
    let mut pairs = Parser::parse(Rule::contact, contact)?;
    if let Some(contact) = pairs.next() {
        if let Some(c) = parse_contact_pair(contact) {
            return c;
        }
    }
    Err(Error::Empty)
}
