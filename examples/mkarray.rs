extern crate serde_json;
extern crate ql2;
extern crate protobuf;

use ql2::proto::Term_TermType as tt;
use serde_json::{from_str, Value};
use protobuf::core::ProtobufEnum;

fn wrap_arrays(mut val: Value) -> Value {
    if val.is_array() {
        let mut array = Vec::with_capacity(2);
        array.push(Value::I64(tt::MAKE_ARRAY.value() as i64));
        if let Value::Array(vec) = val {
            let mut new_val = Vec::with_capacity(vec.len());
            for v in vec.into_iter() {
                if v.is_array() {
                    new_val.push(wrap_arrays(v));
                } else {
                    new_val.push(v)
                }
            }
            val = Value::Array(new_val);
        }
        array.push(val);
        val = Value::Array(array);
    }
    val
}
        

fn main() {
    let jval: Value = from_str(r#"["names", ["Dominic", "Rush]more"]]"#).unwrap();
    let jval = wrap_arrays(jval);
    let json = serde_json::to_string(&jval).unwrap();
    println!("{}", json);
}
