use super::*;

#[test]

fn another() {
    let ex = "=?UTF-8?Q?Mz*_Baltazar=27s_Lab_//_TODAY_01=2E06_-_?= =?UTF-8?Q?FINISSAGE_WITH_A_SPECIAL_PERFORMANCE_=3A?= =?UTF-8?Q?=3A=3A_Spell=3A_Recognition_by_Zosia_Ho?= =?UTF-8?Q?=C5=82ubowska?= <stefanie.wuschitz@mzbaltazarslaboratory.org>";
    println!("{:?}", parse_contact_list(ex));
}

#[test]
fn it_works() {
    // XXX Use concat!
    let hs = [
        "Garbage:Michael Zeltner <m@niij.org>, luck@dresden.dolls, \
        \"Something\" <aaaa@what.com>, Ötsi <w@oow.co>;",
        "\"RFC5322::Still a pain in 2018\": \"Example; Email: \
        Add@ress.es\" <for@real.example.com>, \"messy@example.net\", \
        Very Messy <horrible@formatting.example.org> ;",
        "koordination@netznetz.net, Kunasek; Heinzi \
        <heinzi@example.org>, <this@is.hell>", "A Group:\
        groupmember1@example.org, \"Member 2\"<member2@example.org>, \
        \"3, Member\" <member3@example.org>;", "Versteckte-Empfaenger:;",
        "Undisclosed-Recipients: <>;"
    ];

    let cs = vec![vec![
        Contact::new_with(Some("Garbage"), None),
        Contact::new_with(Some("Michael Zeltner"), Some("m@niij.org")),
        Contact::new_with(None, Some("luck@dresden.dolls")),
        Contact::new_with(Some("Something"), Some("aaaa@what.com")),
        Contact::new_with(Some("Ötsi"), Some("w@oow.co"))], vec![
        Contact::new_with(Some("RFC5322::Still a pain in 2018"), None),
        Contact::new_with(Some("Example; Email: Add@ress.es"),
        Some("for@real.example.com")), Contact::new_with(None,
        Some("messy@example.net")), Contact::new_with(Some("Very Messy"),
        Some("horrible@formatting.example.org"))], vec![
        Contact::new_with(None, Some("koordination@netznetz.net")),
        Contact::new_with(Some("Kunasek; Heinzi"), Some("heinzi@example.org")),
        Contact::new_with(None, Some("this@is.hell"))], vec![
        Contact::new_with(Some("A Group"), None), Contact::new_with(None,
        Some("groupmember1@example.org")), Contact::new_with(Some("Member 2"),
        Some("member2@example.org")), Contact::new_with(Some("3, Member"),
        Some("member3@example.org"))], vec![
        Contact::new_with(Some("Versteckte-Empfaenger"), None)], vec![
        Contact::new_with(Some("Undisclosed-Recipients"), None)]
    ];
    for (i, ex) in hs.iter().enumerate() {
        for (j, contact) in parse_contact_list(ex).iter().enumerate() {
            let cmp = &cs[i][j];
            println!("{:?}\n{:?}", contact, cmp);
            println!("{:?}", contact.name == cmp.name);
            println!("{:?}", contact.email == cmp.email);
            assert!(contact.deep_eq(&cmp));
        }
    }
}
