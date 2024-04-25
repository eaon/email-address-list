use email_address_list::error::*;
use email_address_list::*;
use std::process::Command;

#[test]
fn big_list_of_naughty_strings() {
    let naughty = Command::new("curl")
        .args(["https://raw.githubusercontent.com/minimaxir/big-list-of-naughty-strings/master/blns.txt"])
        .output()
        .expect("This test depends on curl and network connectivity");

    let mut body = String::from_utf8(naughty.stdout).unwrap();
    for line in body.as_mut_str().lines() {
        println!("{}", line);
        match parse_address_list(&line) {
            Ok(a) => {
                assert!(!a.is_group());
                for contact in a.contacts() {
                    assert!(contact.is_garbage());
                }
            }
            Err(Error::Empty) => {}
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
}

#[test]
fn equality() {
    let literals = [
        concat!(
            r#"Garbage:       enghraifft@example.org (Enw), "#,
            r#"luck@dresden.dolls, "Something"      <aaaa@what.com>, Ötsi "#,
            "<w@oow.co,>;",
        ),
        concat!(
            r#""RFC822::Still a pain in 2018": "Example; Email: "#,
            r#"Add@ress.es" <        for@real.example.com>, "messy@example.net", Very"#,
            " (Invalid) Messy  <horrible@formatting.example.org>;",
        ),
        concat!(
            r#"I Know People <with@very.terrible> (email "clients (that don't care)");"#,
            "<whydidweletpeopletype@emailaddress.es>>; its@bad.idea; its@lsotheinspiration.for; ",
            "this@library",
        ),
        concat!(
            ",koordination@netznetz.net, Kunasek; Heinzi <heinzi@example.org>,",
            " <this@is.hell>,   ,  ,",
        ),
        concat!(
            r#"A Group:groupmember1@example.org,"Member 2" "#,
            r#"<member2@example.org>, "3, Member" (via example mailing list) "#,
            "<list@example.org>;",
        ),
        "Last Name, First Name <'email@addre.ss'>, another@one.two",
        "Versteckte-Empfaenger:; <justkidding@twoaredisclos.ed>, thisis@the.next (One, No More)",
        "Undisclosed-Recipients: <>;",
        "<Undisclosed-Recipients: <>;>",
        "<Undisclosed-Recipients:;>",
        "inventing@new.email.addresses.for.fun>, please@make.it <please@make.it >, stop@stop.com",
        concat!(
            r#""I wish email would go away" <go@wa.y>; butitwont@foralong.time; "#,
            r#"It's really sad <imo@opinion.mine>"#
        ),
        "''Hello'' <hello@email.rofl>, <something@okokok.xxq;icant@any.more>, So Much<>bad@stu.ff ",
        concat!(
            "<oneaddress@without.delimiter> <twoaddresses@without.delimiter>, ",
            "oneaddress@with.delimiter",
        ),
    ];
    let address_lists: Vec<AddressList> = vec![
        Group::new("Garbage")
            .set_contacts(vec![
                Contact::new("enghraifft@example.org").set_name("Enw"),
                Contact::new("luck@dresden.dolls"),
                Contact::new("aaaa@what.com").set_name("Something"),
                Contact::new("w@oow.co").set_name("Ötsi"),
            ])
            .into(),
        Group::new("RFC822::Still a pain in 2018")
            .set_contacts(vec![
                Contact::new("for@real.example.com").set_name("Example; Email: Add@ress.es"),
                Contact::new("messy@example.net"),
                Contact::new("horrible@formatting.example.org").set_name("Very (Invalid) Messy"),
            ])
            .into(),
        vec![
            Contact::new("with@very.terrible")
                .set_name("I Know People")
                .set_comment(r#"email "clients (that don't care)""#),
            Contact::new("whydidweletpeopletype@emailaddress.es"),
            Contact::new("its@bad.idea"),
            Contact::new("its@lsotheinspiration.for"),
            Contact::new("this@library"),
        ]
        .into(),
        vec![
            Contact::new("koordination@netznetz.net"),
            Contact::new("heinzi@example.org").set_name("Kunasek; Heinzi"),
            Contact::new("this@is.hell"),
        ]
        .into(),
        Group::new("A Group")
            .set_contacts(vec![
                Contact::new("groupmember1@example.org"),
                Contact::new("member2@example.org").set_name("Member 2"),
                Contact::new("list@example.org")
                    .set_name("3, Member")
                    .set_comment("via example mailing list"),
            ])
            .into(),
        vec![
            Contact::new("email@addre.ss").set_name("Last Name, First Name"),
            Contact::new("another@one.two"),
        ]
        .into(),
        Group::new("Versteckte-Empfaenger")
            .set_contacts(vec![
                Contact::new("justkidding@twoaredisclos.ed"),
                Contact::new("thisis@the.next").set_name("One, No More"),
            ])
            .into(),
        Group::new("Undisclosed-Recipients").into(),
        Group::new("Undisclosed-Recipients").into(),
        Group::new("Undisclosed-Recipients").into(),
        vec![
            Contact::new("inventing@new.email.addresses.for.fun"),
            Contact::new("please@make.it"),
            Contact::new("stop@stop.com"),
        ]
        .into(),
        vec![
            Contact::new("go@wa.y").set_name("I wish email would go away"),
            Contact::new("butitwont@foralong.time"),
            Contact::new("imo@opinion.mine").set_name("It's really sad"),
        ]
        .into(),
        vec![
            Contact::new("hello@email.rofl").set_name("Hello"),
            Contact::new("something@okokok.xxq"),
            Contact::new("icant@any.more"),
            Contact::new("bad@stu.ff").set_name("So Much"),
        ]
        .into(),
        vec![
            Contact::new("oneaddress@without.delimiter"),
            Contact::new("twoaddresses@without.delimiter"),
            Contact::new("oneaddress@with.delimiter"),
        ]
        .into(),
    ];
    assert!(literals.len() == address_lists.len());
    for (i, address_list) in address_lists.iter().enumerate() {
        let j = match i {
            0..=5 => i + 3,
            _ => i - 3,
        };
        let mut other = parse_address_list(literals[i]).unwrap();
        assert!(
            address_list.deep_eq(&other),
            "    is == {:?}\nshould == {:?}\n",
            &address_list,
            &other
        );
        assert_eq!(address_list, &other);
        other = parse_address_list(literals[j]).unwrap();
        assert!(address_list != &other, "!= {:?}", other);
    }
}
