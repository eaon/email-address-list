# email-address-list

[![Build status](https://img.shields.io/appveyor/ci/eaon/email-address-list.svg)](https://ci.appveyor.com/project/eaon/email-address-list)
[![Crate version](https://img.shields.io/crates/v/email-address-list.svg)](https://crates.io/crates/email-address-list)
[![License](https://img.shields.io/crates/l/email-address-list.svg)](https://ghom.niij.org/eaon/email-address-list/src/master/LICENSE)

[Pest based parser](https://pest.rs/) picking out "contacts" from email address
lists found in headers such as `from`, `to`, `cc`, etc.

This library aims to be practical rather than "correct". It is (potentially
excessively) permissive to parse even the worst garbage in everyone's inbox.
Limited testing with real world data was done, but the grammar probably still
needs work to catch more edge cases.

Contacts may have names, email addresses and comments. Groups are parsed as
contacts that have names but no email address or comment. They don't envelop
another set of contacts, everything is flat.

## Thanks

* The [big list of naughty strings](https://github.com/minimaxir/big-list-of-naughty-strings)
  makes testing with horrible input a bit less tedious. 🎊
