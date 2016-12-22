pub use ql2::types as data;

use ::Client;
use ql2::proto::Term;

pub trait IntoArray {
    fn into_array(self) -> data::Array;
}

pub trait IntoBool {
    fn into_bool(self) -> data::Bool;
}

impl<O> IntoBool for Client<data::Bool, O> {
    fn into_bool(self) -> data::Bool {
        self.cmd.0
    }
}

impl<O> IntoBool for Client<data::Null, O> {
    fn into_bool(self) -> data::Bool {
        Term::from_json(false).into()
    }
}

pub trait IntoDb {
    fn into_db(self) -> data::Db;
}

pub trait IntoFunction {
    fn into_function(self) -> data::Function;
}

pub trait IntoGroupedData {
    fn into_grouped_data(self) -> data::GroupedData;
}

pub trait IntoGroupedStream {
    fn into_grouped_stream(self) -> data::GroupedStream;
}

pub trait IntoMaxVal {
    fn into_max_val(self) -> data::MaxVal;
}

pub trait IntoMinVal {
    fn into_min_val(self) -> data::MinVal;
}

pub trait IntoNull {
    fn into_null(self) -> data::Null;
}

pub trait IntoNumber {
    fn into_number(self) -> data::Number;
}

impl IntoNumber for f64 {
    fn into_number(self) -> data::Number {
        Term::from_json(self).into()
    }
}

impl<O> IntoNumber for Client<data::Number, O>
{
    fn into_number(self) -> data::Number {
        self.cmd.0
    }
}

pub trait IntoObject {
    fn into_object(self) -> data::Object;
}

pub trait IntoBinary {
    fn into_binary(self) -> data::Binary;
}

pub trait IntoGeometry {
    fn into_geometry(self) -> data::Geometry;
}

pub trait IntoTime {
    fn into_time(self) -> data::Time;
}

pub trait IntoStream {
    fn into_stream(self) -> data::Stream;
}

pub trait IntoString {
    fn into_string(self) -> data::String;
}

impl IntoString for String {
    fn into_string(self) -> data::String {
        Term::from_json(self).into()
    }
}

impl<'a> IntoString for &'a String {
    fn into_string(self) -> data::String {
        Term::from_json(self).into()
    }
}

impl<'a> IntoString for &'a str {
    fn into_string(self) -> data::String {
        Term::from_json(self).into()
    }
}

impl<O> IntoString for Client<data::String, O>
{
    fn into_string(self) -> data::String {
        self.cmd.0
    }
}

pub trait IntoTable {
    fn into_table(self) -> data::Table;
}

pub trait IntoTableSlice {
    fn into_table_slice(self) -> data::TableSlice;
}

pub trait IntoObjectSelection {
    fn into_object_selection(self) -> data::ObjectSelection;
}

pub trait IntoArraySelection {
    fn into_array_selection(self) -> data::ArraySelection;
}

pub trait IntoStreamSelection {
    fn into_stream_selection(self) -> data::StreamSelection;
}

pub trait IntoPrimaryKey {
    fn into_primary_key(self) -> Term;
}

impl<O> IntoPrimaryKey for Client<data::String, O> {
    fn into_primary_key(self) -> Term {
        self.cmd.0.into()
    }
}

impl IntoPrimaryKey for String {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl<'a> IntoPrimaryKey for &'a String {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl<'a> IntoPrimaryKey for &'a str {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl<O> IntoPrimaryKey for Client<data::Number, O> {
    fn into_primary_key(self) -> Term {
        self.cmd.0.into()
    }
}

impl IntoPrimaryKey for f32 {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoPrimaryKey for i32 {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoPrimaryKey for u32 {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoPrimaryKey for f64 {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoPrimaryKey for i64 {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoPrimaryKey for u64 {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl<O> IntoPrimaryKey for Client<data::Bool, O> {
    fn into_primary_key(self) -> Term {
        self.cmd.0.into()
    }
}

impl IntoPrimaryKey for bool {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

pub trait IntoSecondaryKey : IntoPrimaryKey where Self: Sized {
    fn into_secondary_key(self) -> Term {
        self.into_primary_key()
    }
}

impl<O> IntoSecondaryKey for Client<data::String, O> {}

impl IntoSecondaryKey for String {}

impl<'a> IntoSecondaryKey for &'a String {}

impl<'a> IntoSecondaryKey for &'a str {}

impl<O> IntoSecondaryKey for Client<data::Number, O> {}

impl IntoSecondaryKey for f32 {}

impl IntoSecondaryKey for i32 {}

impl IntoSecondaryKey for u32 {}

impl IntoSecondaryKey for f64 {}

impl IntoSecondaryKey for i64 {}

impl IntoSecondaryKey for u64 {}

impl<O> IntoSecondaryKey for Client<data::Bool, O> {}

impl IntoSecondaryKey for bool {}

pub trait IntoSequence {
    fn into_sequence(self) -> Term;
}

pub trait IntoTerm {
    fn into_term(self) -> Term;
}

impl IntoTerm for String {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl<'a> IntoTerm for &'a String {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl<'a> IntoTerm for &'a str {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl<O> IntoTerm for Client<data::Number, O> {
    fn into_term(self) -> Term {
        self.cmd.0.into()
    }
}

impl IntoTerm for f32 {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoTerm for i32 {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoTerm for u32 {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoTerm for f64 {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoTerm for i64 {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoTerm for u64 {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl<O> IntoTerm for Client<data::Bool, O> {
    fn into_term(self) -> Term {
        self.cmd.0.into()
    }
}

impl IntoTerm for bool {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}
