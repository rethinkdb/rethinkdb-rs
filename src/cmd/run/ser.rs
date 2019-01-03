use std::io::{Result, Write};

use serde::Serialize;
use serde_json::ser::{Formatter, Serializer};

pub(crate) struct ReqlFormatter;

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

pub(crate) fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

pub fn to_writer<W, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
    W: Write,
    T: Serialize,
{
    let mut ser = Serializer::with_formatter(writer, ReqlFormatter);
    value.serialize(&mut ser)?;
    Ok(())
}
