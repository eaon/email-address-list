email-address-list
==================

[![Crate version](https://img.shields.io/crates/v/email-address-list.svg)](https://crates.io/crates/email-address-list)
[![Documentation](https://docs.rs/email-address-list/badge.svg)](https://docs.rs/email-address-list)
[![License](https://img.shields.io/crates/l/email-address-list.svg)](LICENSE)

Relatively naïve [Pest](https://pest.rs/) based parser, picking out "contacts" from "email address
lists" found in headers such as `from`, `to`, `cc`, etc.

This library aims to be practical rather than "correct". It is (potentially excessively) permissive
in parsing even the worst garbage in everyone's inbox. Limited testing with real world data was
done, but the grammar probably still needs work to catch even more edge cases.

`0.0.x` releases may contain bugfixes _and_ features, `0.x.0` might break compatibility.

## Examples

RFC compliant header:

```rust
use email_address_list::*;

let manual: AddressList = vec![
    Contact::new("ríomhphost@example.org").set_name("Túsainm Sloinne"),
    Contact::new("sampla@example.org")
].into();

let result = parse_address_list(
    "Túsainm Sloinne <ríomhphost@example.org>, sampla@example.org"
).unwrap();

assert!(result.deep_eq(&manual));
```

Non RFC compliant header:

```rust
let manual: AddressList = vec![
    Contact::new("enaslov@example.org").set_name("Ime Priimek"),
    Contact::new("primer@example.org"),
    Contact::new("nepravilno.oblikovan@example.org")
        .set_name("Oblikovan, Nepravilno"),
    Contact::new("napačno.oblikovan@example.org"),
].into();

let result = parse_address_list(
    concat!(
        r#""Ime Priimek" <enaslov@example.org;primer@example.org>, "#,
        "Oblikovan, Nepravilno <nepravilno.oblikovan@example.org,>>, ",
        "<'napačno.oblikovan@example.org'>",
    )
).unwrap();

assert!(result.deep_eq(&manual));
```

If you find examples of `email-address-list` failing, either by omitting addresses or supplying
wrong addresses, please share them with the author.

For further information, please see the [documentation](https://docs.rs/email-address-list).

Thanks
------

* The [big list of naughty strings](https://github.com/minimaxir/big-list-of-naughty-strings)
  makes testing with horrible input a bit less tedious. 🎊
