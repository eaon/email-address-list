# email-address-list

[![Build status](https://img.shields.io/appveyor/ci/eaon/email-address-list.svg)](https://ci.appveyor.com/project/eaon/email-address-list)
[![Crate version](https://img.shields.io/crates/v/email-address-list.svg)](https://crates.io/crates/email-address-list)
[![Documentation](https://docs.rs/email-address-list/badge.svg)](https://docs.rs/email-address-list)
[![License](https://img.shields.io/crates/l/email-address-list.svg)](https://ghom.niij.org/eaon/email-address-list/src/master/LICENSE)

Naïve [Pest](https://pest.rs/) based parser, picking out "contacts" from "email
address lists" found in headers such as `from`, `to`, `cc`, etc.

This library aims to be practical rather than "correct". It is (potentially
excessively) permissive in parsing even the worst garbage in everyone's inbox.
Limited testing with real world data was done, but the grammar probably still
needs work to catch more edge cases.

For further information, please see the [documentation](https://docs.rs/email-address-list).

## Thanks

* The [big list of naughty strings](https://github.com/minimaxir/big-list-of-naughty-strings)
  makes testing with horrible input a bit less tedious. 🎊
