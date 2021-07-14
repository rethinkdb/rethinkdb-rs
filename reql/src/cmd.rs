pub mod add;
pub mod and;
pub mod append;
pub mod args;
pub mod asc;
pub mod avg;
pub mod between;
pub mod binary;
pub mod bit_and;
pub mod bit_not;
pub mod bit_or;
pub mod bit_sal;
pub mod bit_sar;
pub mod bit_xor;
pub mod bracket;
pub mod branch;
pub mod ceil;
pub mod change_at;
pub mod changes;
pub mod circle;
pub mod close;
pub mod coerce_to;
pub mod concat_map;
pub mod config;
pub mod connect;
pub mod contains;
pub mod count;
pub mod date;
pub mod day;
pub mod day_of_week;
pub mod day_of_year;
pub mod db;
pub mod db_create;
pub mod db_drop;
pub mod db_list;
pub mod default;
pub mod delete;
pub mod delete_at;
pub mod desc;
pub mod difference;
pub mod distance;
pub mod distinct;
pub mod div;
pub mod do_;
pub mod downcase;
pub mod during;
pub mod epoch_time;
pub mod eq;
pub mod eq_join;
pub mod error;
pub mod expr;
pub mod fill;
pub mod filter;
pub mod floor;
pub mod fold;
pub mod for_each;
pub(crate) mod func;
pub mod ge;
pub mod geojson;
pub mod get;
pub mod get_all;
pub mod get_field;
pub mod get_intersecting;
pub mod get_nearest;
pub mod get_write_hook;
pub mod grant;
pub mod group;
pub mod gt;
pub mod has_fields;
pub mod hours;
pub mod http;
pub mod in_timezone;
pub mod includes;
pub mod index;
pub mod index_create;
pub mod index_drop;
pub mod index_list;
pub mod index_rename;
pub mod index_status;
pub mod index_wait;
pub mod info;
pub mod inner_join;
pub mod insert;
pub mod insert_at;
pub mod intersects;
pub mod is_empty;
pub mod iso8601;
pub mod js;
pub mod json;
pub mod keys;
pub mod le;
pub mod limit;
pub mod line;
pub mod literal;
pub mod lt;
pub mod map;
pub mod match_;
pub mod max;
pub mod merge;
pub mod min;
pub mod minutes;
pub mod month;
pub mod mul;
pub mod ne;
pub mod not;
pub mod now;
pub mod nth;
pub mod object;
pub mod offsets_of;
pub mod or;
pub mod order_by;
pub mod outer_join;
pub mod pluck;
pub mod point;
pub mod polygon;
pub mod polygon_sub;
pub mod prepend;
pub mod random;
pub mod range;
pub mod rebalance;
pub mod reconfigure;
pub mod reduce;
pub mod rem;
pub mod replace;
pub mod round;
pub mod run;
pub mod sample;
pub mod seconds;
pub mod set_difference;
pub mod set_insert;
pub mod set_intersection;
pub mod set_union;
pub mod set_write_hook;
pub mod skip;
pub mod slice;
pub mod splice_at;
pub mod split;
pub mod status;
pub mod sub;
pub mod sum;
pub mod sync;
pub mod table;
pub mod table_create;
pub mod table_drop;
pub mod table_list;
pub mod time;
pub mod time_of_day;
pub mod timezone;
pub mod to_epoch_time;
pub mod to_geojson;
pub mod to_iso8601;
pub mod to_json;
pub mod type_of;
pub mod ungroup;
pub mod union;
pub mod upcase;
pub mod update;
pub mod uuid;
pub mod values;
pub mod wait;
pub mod with_fields;
pub mod without;
pub mod year;
pub mod zip;

use crate::Command;
use futures::stream::Stream;
use ql2::term::TermType;
use serde::de::DeserializeOwned;
use serde::{Serialize, Serializer};
use std::borrow::Cow;
use std::str;

pub use crate::proto::Arg;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ReturnChanges {
    Bool(bool),
    Always,
}

impl Serialize for ReturnChanges {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bool(boolean) => boolean.serialize(serializer),
            Self::Always => "always".serialize(serializer),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Durability {
    Hard,
    Soft,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ReadMode {
    Single,
    Majority,
    Outdated,
}

pub trait StaticString {
    fn static_string(self) -> Cow<'static, str>;
}

impl StaticString for &'static str {
    fn static_string(self) -> Cow<'static, str> {
        Cow::from(self)
    }
}

impl StaticString for String {
    fn static_string(self) -> Cow<'static, str> {
        Cow::from(self)
    }
}

impl StaticString for &Cow<'static, str> {
    fn static_string(self) -> Cow<'static, str> {
        match self {
            Cow::Borrowed(string) => Cow::Borrowed(*string),
            Cow::Owned(string) => Cow::Owned(string.to_owned()),
        }
    }
}

fn debug(bytes: &[u8]) -> String {
    if let Ok(string) = str::from_utf8(bytes) {
        return string.to_owned();
    }
    format!("{:?}", bytes)
}

impl<'a> Command {
    pub fn changes<T>(self, arg: T) -> Self
    where
        T: changes::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    /// Create a table
    ///
    /// A RethinkDB table is a collection of JSON documents.
    ///
    /// ## Example
    ///
    /// Create a table named "dc_universe" with the default settings.
    ///
    /// ```
    /// # reql::example(|r, conn| async_stream::stream! {
    /// r.db("heroes").table_create("dc_universe").run(conn)
    /// # });
    /// ```
    /** ```json
    {
        "config_changes": [
            {
                "new_val": {
                    "db": "test",
                    "durability":  "hard",
                    "id": "20ea60d4-3b76-4817-8828-98a236df0297",
                    "name": "dc_universe",
                    "primary_key": "id",
                    "shards": [
                        {
                            "primary_replica": "rethinkdb_srv1",
                            "replicas": [
                                "rethinkdb_srv1",
                                "rethinkdb_srv2"
                            ]
                        }
                    ],
                    "write_acks": "majority"
                },
                "old_val": None
            }
        ],
        "tables_created": 1
    }
        ```
         */
    pub fn table_create<T>(self, arg: T) -> Self
    where
        T: table_create::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn table_drop<T>(self, arg: T) -> Self
    where
        T: table_drop::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn table_list(self) -> Self {
        Self::new(TermType::TableList).with_parent(self)
    }

    pub fn table<T>(self, arg: T) -> Self
    where
        T: table::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn index_create<T>(self, arg: T) -> Self
    where
        T: index_create::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn index_drop<T>(self, arg: T) -> Self
    where
        T: index_drop::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn index_list(self) -> Self {
        Self::new(TermType::IndexList).with_parent(self)
    }

    pub fn index_rename<T>(self, arg: T) -> Self
    where
        T: index_rename::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn index_status<T>(self, arg: T) -> Self
    where
        T: index_status::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn index_wait<T>(self, arg: T) -> Self
    where
        T: index_wait::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn set_write_hook<T>(self, arg: T) -> Self
    where
        T: set_write_hook::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn get_write_hook(self) -> Self {
        Self::new(TermType::GetWriteHook).with_parent(self)
    }

    pub fn insert<T>(self, arg: T) -> Self
    where
        T: insert::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn update<T>(self, arg: T) -> Self
    where
        T: update::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn replace<T>(self, arg: T) -> Self
    where
        T: replace::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn delete<T>(self, arg: T) -> Self
    where
        T: delete::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn sync(self) -> Self {
        Self::new(TermType::Sync).with_parent(self)
    }

    pub fn get<T>(self, arg: T) -> Self
    where
        T: get::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn get_all<T>(self, arg: T) -> Self
    where
        T: get_all::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn between<T>(self, arg: T) -> Self
    where
        T: between::Arg<'a>,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn filter<T>(self, arg: T) -> Self
    where
        T: filter::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn inner_join<T>(self, arg: T) -> Self
    where
        T: inner_join::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn outer_join<T>(self, arg: T) -> Self
    where
        T: outer_join::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn eq_join<T>(self, arg: T) -> Self
    where
        T: eq_join::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn zip(self) -> Self {
        Self::new(TermType::Zip).with_parent(self)
    }

    pub fn map<T>(self, arg: T) -> Self
    where
        T: map::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn with_fields<T>(self, arg: T) -> Self
    where
        T: with_fields::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn concat_map<T>(self, arg: T) -> Self
    where
        T: concat_map::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    /// Orders the result based on the given column.
    ///
    /// Argument can either be a string, `r.asc("column")` for ascending or `r.desc("column")` for descending.
    /// If the given argument is a string, the direction will default to ascending.
    ///
    /// ## Example
    ///
    /// Sort the result in descending order based on the `created_at` column.
    /// ```
    /// # reql::example(|r, conn| async_stream::stream! {
    /// r.db("database").table("users").order_by(r.desc("created_at")).run(conn)
    /// # });
    /// ```
    ///
    pub fn order_by<T>(self, arg: T) -> Self
    where
        T: order_by::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn skip<T>(self, arg: T) -> Self
    where
        T: skip::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }


    /// Limits the result set to the given amount.
    ///
    /// Argument can be an isize.
    ///
    /// ## Example
    ///
    /// Get 5 records.
    /// ```
    /// # reql::example(|r, conn| async_stream::stream! {
    /// r.db("database").table("users").limit(5).run(conn)
    /// # });
    /// ```
    ///
    pub fn limit<T>(self, arg: T) -> Self
    where
        T: limit::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn slice<T>(self, arg: T) -> Self
    where
        T: slice::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn nth<T>(self, arg: T) -> Self
    where
        T: nth::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn offsets_of<T>(self, arg: T) -> Self
    where
        T: offsets_of::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn is_empty(self) -> Self {
        Self::new(TermType::IsEmpty).with_parent(self)
    }

    pub fn union<T>(self, arg: T) -> Self
    where
        T: union::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn sample<T>(self, arg: T) -> Self
    where
        T: sample::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn group<T>(self, arg: T) -> Self
    where
        T: group::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn ungroup(self) -> Self {
        Self::new(TermType::Ungroup).with_parent(self)
    }

    pub fn reduce<T>(self, arg: T) -> Self
    where
        T: reduce::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn fold<T>(self, arg: T) -> Self
    where
        T: fold::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn count<T>(self, arg: T) -> Self
    where
        T: count::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn sum<T>(self, arg: T) -> Self
    where
        T: sum::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn avg<T>(self, arg: T) -> Self
    where
        T: avg::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn min<T>(self, arg: T) -> Self
    where
        T: min::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn max<T>(self, arg: T) -> Self
    where
        T: max::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn distinct<T>(self, arg: T) -> Self
    where
        T: distinct::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn contains<T>(self, arg: T) -> Self
    where
        T: contains::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn pluck<T>(self, arg: T) -> Self
    where
        T: pluck::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn without<T>(self, arg: T) -> Self
    where
        T: without::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn merge<T>(self, arg: T) -> Self
    where
        T: merge::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn append<T>(self, arg: T) -> Self
    where
        T: append::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn prepend<T>(self, arg: T) -> Self
    where
        T: prepend::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn difference<T>(self, arg: T) -> Self
    where
        T: difference::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn set_insert<T>(self, arg: T) -> Self
    where
        T: set_insert::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn set_union<T>(self, arg: T) -> Self
    where
        T: set_union::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn set_intersection<T>(self, arg: T) -> Self
    where
        T: set_intersection::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn set_difference<T>(self, arg: T) -> Self
    where
        T: set_difference::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bracket<T>(self, arg: T) -> Self
    where
        T: bracket::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn get_field<T>(self, arg: T) -> Self
    where
        T: get_field::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn has_fields<T>(self, arg: T) -> Self
    where
        T: has_fields::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn insert_at<T>(self, arg: T) -> Self
    where
        T: insert_at::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn splice_at<T>(self, arg: T) -> Self
    where
        T: splice_at::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn delete_at<T>(self, arg: T) -> Self
    where
        T: delete_at::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn change_at<T>(self, arg: T) -> Self
    where
        T: change_at::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn keys(self) -> Self {
        Self::new(TermType::Keys).with_parent(self)
    }

    pub fn values(self) -> Self {
        Self::new(TermType::Values).with_parent(self)
    }

    pub fn match_<T>(self, arg: T) -> Self
    where
        T: match_::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn split<T>(self, arg: T) -> Self
    where
        T: split::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn upcase(self) -> Self {
        Self::new(TermType::Upcase).with_parent(self)
    }

    pub fn downcase(self) -> Self {
        Self::new(TermType::Downcase).with_parent(self)
    }

    pub fn and<T>(self, arg: T) -> Self
    where
        T: and::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn or<T>(self, arg: T) -> Self
    where
        T: or::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn eq<T>(self, arg: T) -> Self
    where
        T: eq::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn ne<T>(self, arg: T) -> Self
    where
        T: ne::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn gt<T>(self, arg: T) -> Self
    where
        T: gt::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn ge<T>(self, arg: T) -> Self
    where
        T: ge::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn lt<T>(self, arg: T) -> Self
    where
        T: lt::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn le<T>(self, arg: T) -> Self
    where
        T: le::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn not<T>(self, arg: T) -> Self
    where
        T: not::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bit_and<T>(self, arg: T) -> Self
    where
        T: bit_and::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bit_or<T>(self, arg: T) -> Self
    where
        T: bit_or::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bit_xor<T>(self, arg: T) -> Self
    where
        T: bit_xor::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bit_not(self) -> Self {
        !self
    }

    pub fn bit_sal<T>(self, arg: T) -> Self
    where
        T: bit_sal::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bit_sar<T>(self, arg: T) -> Self
    where
        T: bit_sar::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn round(self) -> Self {
        Self::new(TermType::Round).with_parent(self)
    }

    pub fn ceil(self) -> Self {
        Self::new(TermType::Ceil).with_parent(self)
    }

    pub fn floor(self) -> Self {
        Self::new(TermType::Floor).with_parent(self)
    }

    pub fn in_timezone<T>(self, arg: T) -> Self
    where
        T: in_timezone::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn timezone(self) -> Self {
        Self::new(TermType::Timezone).with_parent(self)
    }

    pub fn during<T>(self, arg: T) -> Self
    where
        T: during::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn date(self) -> Self {
        Self::new(TermType::Date).with_parent(self)
    }

    pub fn time_of_day(self) -> Self {
        Self::new(TermType::TimeOfDay).with_parent(self)
    }

    pub fn year(self) -> Self {
        Self::new(TermType::Year).with_parent(self)
    }

    pub fn month(self) -> Self {
        Self::new(TermType::Month).with_parent(self)
    }

    pub fn day(self) -> Self {
        Self::new(TermType::Day).with_parent(self)
    }

    pub fn day_of_week(self) -> Self {
        Self::new(TermType::DayOfWeek).with_parent(self)
    }

    pub fn day_of_year(self) -> Self {
        Self::new(TermType::DayOfYear).with_parent(self)
    }

    pub fn hours(self) -> Self {
        Self::new(TermType::Hours).with_parent(self)
    }

    pub fn minutes(self) -> Self {
        Self::new(TermType::Minutes).with_parent(self)
    }

    pub fn seconds(self) -> Self {
        Self::new(TermType::Seconds).with_parent(self)
    }

    pub fn to_iso8601(self) -> Self {
        Self::new(TermType::ToIso8601).with_parent(self)
    }

    pub fn to_epoch_time(self) -> Self {
        Self::new(TermType::ToEpochTime).with_parent(self)
    }

    pub fn binary<T>(self, arg: T) -> Self
    where
        T: binary::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn do_<T>(self, arg: T) -> Self
    where
        T: do_::Arg,
    {
        arg.arg(Some(self)).into_cmd()
    }

    pub fn branch<T>(self, arg: T) -> Self
    where
        T: branch::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn for_each<T>(self, arg: T) -> Self
    where
        T: for_each::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn default<T>(self, arg: T) -> Self
    where
        T: default::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn coerce_to<T>(self, arg: T) -> Self
    where
        T: coerce_to::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn type_of(self) -> Self {
        Self::new(TermType::TypeOf).with_parent(self)
    }

    pub fn info(self) -> Self {
        Self::new(TermType::Info).with_parent(self)
    }

    pub fn to_json(self) -> Self {
        Self::new(TermType::ToJsonString).with_parent(self)
    }

    pub fn distance<T>(self, arg: T) -> Self
    where
        T: distance::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn fill(self) -> Self {
        Self::new(TermType::Fill).with_parent(self)
    }

    pub fn to_geojson(self) -> Self {
        Self::new(TermType::ToGeojson).with_parent(self)
    }

    pub fn get_intersecting<T>(self, arg: T) -> Self
    where
        T: get_intersecting::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn get_nearest<T>(self, arg: T) -> Self
    where
        T: get_nearest::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn includes<T>(self, arg: T) -> Self
    where
        T: includes::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn intersects<T>(self, arg: T) -> Self
    where
        T: intersects::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn polygon_sub<T>(self, arg: T) -> Self
    where
        T: polygon_sub::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn grant<T>(self, arg: T) -> Self
    where
        T: grant::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn config(self) -> Self {
        Self::new(TermType::Config).with_parent(self)
    }

    pub fn rebalance(self) -> Self {
        Self::new(TermType::Rebalance).with_parent(self)
    }

    pub fn reconfigure<T>(self, arg: T) -> Self
    where
        T: reconfigure::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn status(self) -> Self {
        Self::new(TermType::Status).with_parent(self)
    }

    pub fn wait<T>(self, arg: T) -> Self
    where
        T: wait::Arg,
    {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn run<A, T>(self, arg: A) -> impl Stream<Item = crate::Result<T>>
    where
        A: run::Arg,
        T: Unpin + DeserializeOwned,
    {
        Box::pin(run::new(self, arg))
    }
}

#[cfg(test)]
fn serialise(cmd: &Command) -> String {
    serde_json::to_string(&crate::Query(cmd)).unwrap()
}
