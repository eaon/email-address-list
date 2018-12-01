# email-address-list

[![Build status](https://img.shields.io/appveyor/ci/eaon/email-address-list.svg)](https://ci.appveyor.com/project/eaon/email-address-list)
[![Crate version](https://img.shields.io/crates/v/email-address-list.svg)](https://crates.io/crates/email-address-list)
[![Documentation](https://docs.rs/email-address-list/badge.svg)](https://docs.rs/email-address-list)
[![License](https://img.shields.io/crates/l/email-address-list.svg)](https://ghom.niij.org/eaon/email-address-list/src/master/LICENSE)

Relatively naïve [Pest](https://pest.rs/) based parser, picking out "contacts"
from "email address lists" found in headers such as `from`, `to`, `cc`, etc.

This library aims to be practical rather than "correct". It is (potentially
excessively) permissive in parsing even the worst garbage in everyone's inbox.
Limited testing with real world data was done, but the grammar probably still
needs work to catch more edge cases.

Since this library is quite young, the API might change in `$version + 0.1`
releases, but minor ones won't break compatibility.

## Example

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

For further information, please see the [documentation](https://docs.rs/email-address-list).

## Thanks

* The [big list of naughty strings](https://github.com/minimaxir/big-list-of-naughty-strings)
  makes testing with horrible input a bit less tedious. 🎊
