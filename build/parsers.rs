named!{
    graves, delimited!(
        // A code block starts with ```javascript
        ws!(pair!(tag!("```"), opt!(tag!("javascript")))),
        // contains the example
        is_not!("```"),
        // and ends with ```
        ws!(tag!("```"))
    )
}
