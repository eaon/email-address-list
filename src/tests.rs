use super::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn address_list_from_file(filename: &str) -> Result<Vec<AddressList>> {
    let mut address_lists = Vec::<AddressList>::new();
    match File::open(filename) {
        Ok(f) => {
            let mut lines = BufReader::new(f);
            let mut line = String::new();
            while lines.read_line(&mut line).unwrap() > 0 {
                let parsed = parse_address_list(&Some(&line))?;
                address_lists.extend(parsed);
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

//#[test]
fn naughty_input() -> Result<()> {
    let address_lists = address_list_from_file("tests/naughty-strings.txt")?;
    for address_list in address_lists {
        println!("{:?}", address_list);
        assert!(false);
    }
    Ok(())
}

#[test]
fn deep_eq() {
//    let als = vec![
//        AddressList::from(
//            Group::new_with(
//                "Garbage",
//                vec![
//                    Contact::new_with(
//                        "m@niij.org",
//                        Some("Michael Zeltner"),
//                        None
//                    )
//                ]
//            )),
//    ];
    match address_list_from_file("tests/deep_eq.txt") {
        Ok(address_lists) => {
            for contact in address_lists {
                println!("{:?}", contact);
                //let cmp = &cs[i];
                //println!("{:?}\n{:?}", contact, cmp);
                //println!("{:?}", contact.name == cmp.name);
                //println!("{:?}", contact.email == cmp.email);
                //assert!(contact.deep_eq(&cmp));
            }
        }
        Err(e) => {
            println!("{}", match e {
                Error::UnexpectedError(e) => e,
                _ => format!("{:?}", e),
            });
        }
    }
    assert!(false);
//        Contact::new_with(Some("Michael Zeltner"), Some("m@niij.org"), None),
//        Contact::new_with(None, Some("luck@dresden.dolls"), None),
//        Contact::new_with(Some("Something"), Some("aaaa@what.com"), None),
//        Contact::new_with(Some("Ötsi"), Some("w@oow.co"), None),
//        Contact::new_with(Some("RFC5322::Still a pain in 2018"), None, None),
//        Contact::new_with(
//            Some("Example; Email: Add@ress.es"),
//            Some("for@real.example.com"),
//            None,
//        ),
//        Contact::new_with(None, Some("messy@example.net"), None),
//        Contact::new_with(
//            Some("Very (Invalid) Messy"),
//            Some("horrible@formatting.example.org"),
//            None,
//        ),
//        Contact::new_with(None, Some("koordination@netznetz.net"), None),
//        Contact::new_with(
//            Some("Kunasek; Heinzi"),
//            Some("heinzi@example.org"),
//            None,
//        ),
//        Contact::new_with(None, Some("this@is.hell"), None),
//        Contact::new_with(Some("A Group"), None, None),
//        Contact::new_with(None, Some("groupmember1@example.org"), None),
//        Contact::new_with(Some("Member 2"), Some("member2@example.org"), None),
//        Contact::new_with(
//            Some("3, Member"),
//            Some("list@example.org"),
//            Some("via example mailing list"),
//        ),
//        Contact::new_with(
//            Some("Last Name, First Name"),
//            Some("email@addre.ss"),
//            None,
//        ),
//        Contact::new_with(None, Some("another@one.two"), None),
//        Contact::new_with(Some("Versteckte-Empfaenger"), None, None),
//        Contact::new_with(Some("Undisclosed-Recipients"), None, None),
//        Contact::new_with(Some("Undisclosed-Recipients"), None, None),
//        Contact::new_with(Some("Undisclosed-Recipients"), None, None),
}
