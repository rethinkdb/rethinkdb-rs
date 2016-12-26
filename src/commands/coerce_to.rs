#![allow(dead_code)]

use {Client, types};
use args::term::IntoTerm;
use serde_json::value::ToJson;
use ql2::proto::Term_TermType as TermType;

macro_rules! coerce {
    ($typ:ident to $new_typ:ident using $func:ident) => {
        impl<O> Client<types::$typ, O>
            where O: ToJson + Clone
            {
                pub fn $func(self) -> Client<types::$new_typ, ()>
                {
                    let arg = stringify!($new_typ).to_lowercase();
                    super::client(TermType::COERCE_TO, Some(vec![arg.into_term()]), None, self)
                }
            }
    }
}

coerce!{ Number to String using coerce_to_string }
coerce!{ Stream to Array using coerce_to_array }
coerce!{ String to Number using coerce_to_number }
coerce!{ Array to Object using coerce_to_object }
coerce!{ Object to Array using coerce_to_array }
coerce!{ Binary to String using coerce_to_string }
coerce!{ String to Binary using coerce_to_binary }
