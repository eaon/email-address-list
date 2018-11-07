use super::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn contacts_from_file(filename: &str) -> Vec<Contact> {
    let mut contacts = Vec::<Contact>::new();
    match File::open(filename) {
        Ok(f) => {
            let mut lines = BufReader::new(f);
            let mut line = String::new();
            while lines.read_line(&mut line).unwrap() > 0 {
                let parsed = parse_contact_list(Some(&line));
                println!("{:?}", parsed);
                contacts.extend(parsed);
                line.clear();
            }
        },
        Err(error) => {
            println!("Couldn't open file {}", error);
            assert!(false);
        }
    }
    contacts
}

#[test]
fn naughty_input() {
    let contacts = contacts_from_file("tests/naughty-strings.txt");
    for contact in contacts {
        assert!(!contact.email.is_some())
    }
}

#[test]
fn deep_eq() {
    let contacts = contacts_from_file("tests/deep_eq.txt");
    let cs = vec![
        Contact::new_with(Some("Garbage"), None, None),
        Contact::new_with(Some("Michael Zeltner"), Some("m@niij.org"), None),
        Contact::new_with(None, Some("luck@dresden.dolls"), None),
        Contact::new_with(Some("Something"), Some("aaaa@what.com"), None),
        Contact::new_with(Some("Ötsi"), Some("w@oow.co"), None),
        Contact::new_with(Some("RFC5322::Still a pain in 2018"), None, None),
        Contact::new_with(Some("Example; Email: Add@ress.es"),
        Some("for@real.example.com"), None), Contact::new_with(None,
        Some("messy@example.net"), None),
        Contact::new_with(Some("Very (Invalid) Messy"),
        Some("horrible@formatting.example.org"), None),
        Contact::new_with(None, Some("koordination@netznetz.net"), None),
        Contact::new_with(Some("Kunasek; Heinzi"),
        Some("heinzi@example.org"), None), Contact::new_with(None,
        Some("this@is.hell"), None), Contact::new_with(Some("A Group"), None,
        None), Contact::new_with(None, Some("groupmember1@example.org"),
        None), Contact::new_with(Some("Member 2"),
        Some("member2@example.org"), None),
        Contact::new_with(Some("3, Member"), Some("list@example.org"),
        Some("via example mailing list")),
        Contact::new_with(Some("Last Name, First Name"),
        Some("email@addre.ss"), None), Contact::new_with(None,
        Some("another@one.two"), None),
        Contact::new_with(Some("Versteckte-Empfaenger"), None, None),
        Contact::new_with(Some("Undisclosed-Recipients"), None, None),
        Contact::new_with(Some("Undisclosed-Recipients"), None, None),
        Contact::new_with(Some("Undisclosed-Recipients"), None, None)
    ];
    for (i, contact) in contacts.iter().enumerate() {
        let cmp = &cs[i];
        println!("{:?}\n{:?}", contact, cmp);
        println!("{:?}", contact.name == cmp.name);
        println!("{:?}", contact.email == cmp.email);
        assert!(contact.deep_eq(&cmp));
    }
}
