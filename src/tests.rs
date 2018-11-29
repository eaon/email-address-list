use super::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn address_lists_from_file(filename: &str) -> Result<Vec<AddressList>> {
    let mut address_lists = Vec::<AddressList>::new();
    match File::open(filename) {
        Ok(f) => {
            let mut lines = BufReader::new(f);
            let mut line = String::new();
            while lines.read_line(&mut line).unwrap() > 0 {
                if let Some(parsed) = match parse_address_list(&Some(&line)) {
                    Ok(a) => Some(a),
                    Err(Error::Empty) => None,
                    Err(e) => return Err(e),
                } {
                    address_lists.push(parsed);
                }
                line.clear();
            }
        }
        Err(error) => {
            println!("Couldn't open file {}", error);
            assert!(false);
        }
    }
    Ok(address_lists)
}

#[test]
fn naughty_input() {
    let address_lists =
        address_lists_from_file("tests/naughty-strings.txt").unwrap();
    for address_list in address_lists {
        match address_list {
            AddressList::Contacts(c) => {
                for contact in c {
                    match contact {
                        Contact::Garbage(_) => {}
                        _ => {
                            println!("{:?}", contact);
                            assert!(false);
                        }
                    }
                }
            }
            _ => {
                println!("{:?}", address_list);
                assert!(false);
            }
        }
    }
}

#[test]
fn eq() {
    let als = vec![
        AddressList::from(Group::new_with(
            "Garbage",
            vec![
                Contact::new_with("m@niij.org", &[Some("Michael Zeltner")]),
                Contact::new_with("luck@dresden.dolls", &[]),
                Contact::new_with("aaaa@what.com", &[Some("Something")]),
                Contact::new_with("w@oow.co", &[Some("Ötsi")]),
            ],
        )),
        AddressList::from(Group::new_with(
            "RFC5322::Still a pain in 2018",
            vec![
                Contact::new_with(
                    "for@real.example.com",
                    &[Some("Example; Email: Add@ress.es")],
                ),
                Contact::new_with("messy@example.net", &[]),
                Contact::new_with(
                    "horrible@formatting.example.org",
                    &[Some("Very (Invalid) Messy")],
                ),
            ],
        )),
        AddressList::from(vec![
            Contact::new_with("koordination@netznetz.net", &[]),
            Contact::new_with("heinzi@example.org", &[Some("Kunasek; Heinzi")]),
            Contact::new_with("this@is.hell", &[]),
        ]),
        AddressList::from(Group::new_with(
            "A Group",
            vec![
                Contact::new_with("groupmember1@example.org", &[]),
                Contact::new_with("member2@example.org", &[Some("Member 2")]),
                Contact::new_with(
                    "list@example.org",
                    &[Some("3, Member"), Some("via example mailing list")],
                ),
            ],
        )),
        AddressList::from(vec![
            Contact::new_with(
                "email@addre.ss",
                &[Some("Last Name, First Name")],
            ),
            Contact::new_with("another@one.two", &[]),
        ]),
        AddressList::from(Group::new_with("Versteckte-Empfaenger", vec![])),
        AddressList::from(Group::new_with("Undisclosed-Recipients", vec![])),
        AddressList::from(Group::new_with("Undisclosed-Recipients", vec![])),
        AddressList::from(Group::new_with("Undisclosed-Recipients", vec![])),
    ];
    match address_lists_from_file("tests/deep_eq.txt") {
        Ok(address_lists) => {
            for (i, al) in address_lists.iter().enumerate() {
                let j = match i {
                    0...5 => i + 3,
                    _ => i - 3,
                };
                println!("== {:?}\n== {:?}", &al, &als[i]);
                assert!(al.deep_eq(&als[i]));
                assert_eq!(al, &als[i]);
                println!("!= {:?}", als[j]);
                assert!(al != &als[j]);
            }
        }
        Err(e) => {
            println!(
                "{}",
                match e {
                    Error::UnexpectedError(e) => e,
                    _ => format!("{:?}", e),
                }
            );
            assert!(false);
        }
    }
}
