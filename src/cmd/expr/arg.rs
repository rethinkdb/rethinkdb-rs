use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct Arg(pub(super) Bytes);

impl<'a> From<&'a str> for Arg {
    fn from(t: &'a str) -> Self {
        let string = format!("{:?}", t);
        Arg(Bytes::from(string.as_bytes()))
    }
}
