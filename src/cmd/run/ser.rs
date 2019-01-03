use std::io::{Result, Write};

use serde::Serialize;
use serde_json::ser::{Formatter, Serializer};

// Overrides JSON's formatting of arrays to wrap them in the `MAKE_ARRAY`
// command since ReQL uses arrays for serialization
struct ReqlFormatter;

impl Formatter for ReqlFormatter {
    #[inline]
    fn begin_array<W: ?Sized>(&mut self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        writer.write_all(b"[2,[")
    }

    #[inline]
    fn end_array<W: ?Sized>(&mut self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        writer.write_all(b"]]")
    }
}

pub(crate) fn to_vec<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value).unwrap();
    writer
}

fn to_writer<W, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
    W: Write,
    T: Serialize,
{
    let mut ser = Serializer::with_formatter(writer, ReqlFormatter);
    value.serialize(&mut ser)?;
    Ok(())
}
