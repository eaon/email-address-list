/*!
Relatively näive [Pest](https://pest.rs/) based parser, picking out "contacts"
from email address lists found in headers such as `from`, `to`, `cc`, etc.

This library aims to be practical rather than "correct". It is (potentially
excessively) permissive to parse even the worst garbage in anyone's inbox.
Limited testing with real world data has been, but the grammar that forms the
basis for this library probably still needs work to catch more edge cases.

# Example

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
*/

pub mod error;

mod address_list;
pub use crate::address_list::*;

mod parser;
pub use crate::parser::{parse_address_list, parse_contact};
