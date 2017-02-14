use nom::multispace;

named!{
    graves, ws!(delimited!(
        // A code block starts with ```javascript or ```js
        pair!(tag!("```"), alt!(tag!("js\n") | tag!("javascript\n"))),
        // contains the example
        is_not!("```"),
        // and ends with ```
        ws!(tag!("```"))
    ))
}
