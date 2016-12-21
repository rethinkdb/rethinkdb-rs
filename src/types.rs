use ::Client;
use ql2::types;
use ql2::proto::{Term, Datum, Term_TermType as TermType, 
    //Term_AssocPair as TermPair,
    Datum_DatumType as DatumType,
    //Datum_AssocPair as DatumPair,
};
//use protobuf::repeated::RepeatedField;

pub trait IntoArray {
    fn into_array(self) -> types::Array;
}

pub trait IntoBool {
    fn into_bool(self) -> types::Bool;
}

pub trait IntoDb {
    fn into_db(self) -> types::Db;
}

pub trait IntoFunction {
    fn into_function(self) -> types::Function;
}

pub trait IntoGroupedData {
    fn into_grouped_data(self) -> types::GroupedData;
}

pub trait IntoGroupedStream {
    fn into_grouped_stream(self) -> types::GroupedStream;
}

pub trait IntoMaxVal {
    fn into_max_val(self) -> types::MaxVal;
}

pub trait IntoMinVal {
    fn into_min_val(self) -> types::MinVal;
}

pub trait IntoNull {
    fn into_null(self) -> types::Null;
}

pub trait IntoNumber {
    fn into_number(self) -> types::Number;
}

pub trait IntoObject {
    fn into_object(self) -> types::Object;
}

pub trait IntoBinary {
    fn into_binary(self) -> types::Binary;
}

pub trait IntoGeometry {
    fn into_geometry(self) -> types::Geometry;
}

pub trait IntoTime {
    fn into_time(self) -> types::Time;
}

pub trait IntoStream {
    fn into_stream(self) -> types::Stream;
}

pub trait IntoString {
    fn into_string(self) -> types::String;
}

impl IntoString for String {
    fn into_string(self) -> types::String {
        let mut datum = Datum::new();
        datum.set_field_type(DatumType::R_STR);
        datum.set_r_str(self);
        let mut term = Term::new();
        term.set_field_type(TermType::DATUM);
        term.set_datum(datum);
        term.into()
    }
}

impl<'a> IntoString for &'a String {
    fn into_string(self) -> types::String {
        let string = self.to_string();
        string.into_string()
    }
}

impl<'a> IntoString for &'a str {
    fn into_string(self) -> types::String {
        let string = self.to_string();
        string.into_string()
    }
}

impl<O> IntoString for Client<types::String, O>
{
    fn into_string(self) -> types::String {
        self.cmd.0
    }
}

pub trait IntoTable {
    fn into_table(self) -> types::Table;
}

pub trait IntoTableSlice {
    fn into_table_slice(self) -> types::TableSlice;
}

pub trait IntoObjectSelection {
    fn into_object_selection(self) -> types::ObjectSelection;
}

pub trait IntoArraySelection {
    fn into_array_selection(self) -> types::ArraySelection;
}

pub trait IntoStreamSelection {
    fn into_stream_selection(self) -> types::StreamSelection;
}

pub trait PrimaryKey {
    fn into_term(self) -> Term;
}

pub trait SecondaryKey {
    fn into_term(self) -> Term;
}
