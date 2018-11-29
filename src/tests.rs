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
    let als: Vec<AddressList> = vec![
        Group::new("Garbage")
            .set_contacts(vec![
                Contact::new("m@niij.org").set_name("Michael Zeltner"),
                Contact::new("luck@dresden.dolls"),
                Contact::new("aaaa@what.com").set_name("Something"),
                Contact::new("w@oow.co").set_name("Ötsi"),
            ]).into(),
        Group::new("RFC5322::Still a pain in 2018")
            .set_contacts(vec![
                Contact::new("for@real.example.com")
                    .set_name("Example; Email: Add@ress.es"),
                Contact::new("messy@example.net"),
                Contact::new("horrible@formatting.example.org")
                    .set_name("Very (Invalid) Messy"),
            ]).into(),
        vec![
            Contact::new("koordination@netznetz.net"),
            Contact::new("heinzi@example.org").set_name("Kunasek; Heinzi"),
            Contact::new("this@is.hell"),
        ].into(),
        Group::new("A Group")
            .set_contacts(vec![
                Contact::new("groupmember1@example.org"),
                Contact::new("member2@example.org").set_name("Member 2"),
                Contact::new("list@example.org")
                    .set_name("3, Member")
                    .set_comment("via example mailing list"),
            ]).into(),
        vec![
            Contact::new("email@addre.ss").set_name("Last Name, First Name"),
            Contact::new("another@one.two"),
        ].into(),
        Group::new("Versteckte-Empfaenger").into(),
        Group::new("Undisclosed-Recipients").into(),
        Group::new("Undisclosed-Recipients").into(),
        Group::new("Undisclosed-Recipients").into(),
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
