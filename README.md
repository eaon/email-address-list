# email-address-list

[Pest based parser](https://pest.rs/) picking out "contacts" from email address
lists found in headers such as `from`, `to`, `cc`, etc.

This library aims to be practical rather than "correct". It is (potentially
excessively) permissive to parse even the worst garbage in everyone's inbox.
Limited testing with real world data was done, but the grammar probably still
needs work to catch more edge cases.

Also: thanks to the [big list of naughty strings](https://github.com/minimaxir/big-list-of-naughty-strings)
for making testing with horrible input a bit less tedious.
