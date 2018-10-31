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
                let parsed = parse_contact_list(&line);
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
        Contact::new_with(Some("Garbage"), None),
        Contact::new_with(Some("Michael Zeltner"), Some("m@niij.org")),
        Contact::new_with(None, Some("luck@dresden.dolls")),
        Contact::new_with(Some("Something"), Some("aaaa@what.com")),
        Contact::new_with(Some("Ötsi"), Some("w@oow.co")),
        Contact::new_with(Some("RFC5322::Still a pain in 2018"), None),
        Contact::new_with(Some("Example; Email: Add@ress.es"),
        Some("for@real.example.com")), Contact::new_with(None,
        Some("messy@example.net")),
        Contact::new_with(Some("Very (Invalid) Messy"),
        Some("horrible@formatting.example.org")), Contact::new_with(None,
        Some("koordination@netznetz.net")),
        Contact::new_with(Some("Kunasek; Heinzi"),
        Some("heinzi@example.org")), Contact::new_with(None,
        Some("this@is.hell")), Contact::new_with(Some("A Group"), None),
        Contact::new_with(None, Some("groupmember1@example.org")),
        Contact::new_with(Some("Member 2"), Some("member2@example.org")),
        Contact::new_with(Some("3, Member"), Some("member3@example.org")),
        Contact::new_with(Some("Last Name, First Name"),
        Some("email@addre.ss")), Contact::new_with(None,
        Some("another@one.two")),
        Contact::new_with(Some("Versteckte-Empfaenger"), None),
        Contact::new_with(Some("Undisclosed-Recipients"), None),
        Contact::new_with(Some("Undisclosed-Recipients"), None),
        Contact::new_with(Some("Undisclosed-Recipients"), None)
    ];
    for (i, contact) in contacts.iter().enumerate() {
        let cmp = &cs[i];
        println!("{:?}\n{:?}", contact, cmp);
        println!("{:?}", contact.name == cmp.name);
        println!("{:?}", contact.email == cmp.email);
        assert!(contact.deep_eq(&cmp));
    }
}
