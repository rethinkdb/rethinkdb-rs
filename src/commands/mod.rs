
// AUTO GENERATED
// Edit in `build/commands.rs` instead

mod args;
#[cfg(feature = "with_io")]
mod io;
#[cfg(feature = "with_io")]
pub use self::io::*;

use {Client, ToArg};
use ql2::proto::Term;
use protobuf::repeated::RepeatedField;
use ql2::proto::Term_TermType;


fn cmd(name: &str) -> Client {
    unimplemented!();
}



fn cmd_with_args<T: ToArg>(name: &str, args: T) -> Client {
    unimplemented!();
}



impl Client {
    
    pub fn connect(&self) -> Client {
        cmd("connect")
    }


    pub fn close(&self) -> Client {
        cmd("close")
    }


    pub fn reconnect<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("reconnect", args)
    }


    pub fn run<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("run", args)
    }


    pub fn run_noreply<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("run_noreply", args)
    }


    pub fn changes(&self) -> Client {
        cmd("changes")
    }


    pub fn noreply_wait(&self) -> Client {
        cmd("noreply_wait")
    }


    pub fn server(&self) -> Client {
        cmd("server")
    }


    pub fn optarg<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("optarg", args)
    }


    pub fn db_create<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("db_create", args)
    }


    pub fn db_drop<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("db_drop", args)
    }


    pub fn db_list(&self) -> Client {
        cmd("db_list")
    }


    pub fn table_create<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("table_create", args)
    }


    pub fn table_drop<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("table_drop", args)
    }


    pub fn table_list(&self) -> Client {
        cmd("table_list")
    }


    pub fn index_create<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("index_create", args)
    }


    pub fn index_drop<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("index_drop", args)
    }


    pub fn index_list(&self) -> Client {
        cmd("index_list")
    }


    pub fn index_rename<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("index_rename", args)
    }


    pub fn index_status(&self) -> Client {
        cmd("index_status")
    }


    pub fn index_wait(&self) -> Client {
        cmd("index_wait")
    }


    pub fn insert<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("insert", args)
    }


    pub fn update<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("update", args)
    }


    pub fn replace<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("replace", args)
    }


    pub fn delete(&self) -> Client {
        cmd("delete")
    }


    pub fn sync(&self) -> Client {
        cmd("sync")
    }


    pub fn db<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("db", args)
    }


    pub fn table<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("table", args)
    }


    pub fn get<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("get", args)
    }


    pub fn get_all<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("get_all", args)
    }


    pub fn between<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("between", args)
    }


    pub fn filter<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("filter", args)
    }


    pub fn inner_join<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("inner_join", args)
    }


    pub fn outer_join<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("outer_join", args)
    }


    pub fn eq_join<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("eq_join", args)
    }


    pub fn zip(&self) -> Client {
        cmd("zip")
    }


    pub fn map<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("map", args)
    }


    pub fn with_fields<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("with_fields", args)
    }


    pub fn concat_map<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("concat_map", args)
    }


    pub fn order_by(&self) -> Client {
        cmd("order_by")
    }


    pub fn skip<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("skip", args)
    }


    pub fn limit<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("limit", args)
    }


    pub fn slice<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("slice", args)
    }


    pub fn nth<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("nth", args)
    }


    pub fn offsets_of<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("offsets_of", args)
    }


    pub fn is_empty(&self) -> Client {
        cmd("is_empty")
    }


    pub fn union<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("union", args)
    }


    pub fn sample<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("sample", args)
    }


    pub fn group(&self) -> Client {
        cmd("group")
    }


    pub fn ungroup(&self) -> Client {
        cmd("ungroup")
    }


    pub fn reduce<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("reduce", args)
    }


    pub fn fold<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("fold", args)
    }


    pub fn count(&self) -> Client {
        cmd("count")
    }


    pub fn sum(&self) -> Client {
        cmd("sum")
    }


    pub fn avg(&self) -> Client {
        cmd("avg")
    }


    pub fn min(&self) -> Client {
        cmd("min")
    }


    pub fn max(&self) -> Client {
        cmd("max")
    }


    pub fn distinct(&self) -> Client {
        cmd("distinct")
    }


    pub fn contains<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("contains", args)
    }


    pub fn pluck<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("pluck", args)
    }


    pub fn without<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("without", args)
    }


    pub fn merge<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("merge", args)
    }


    pub fn append<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("append", args)
    }


    pub fn prepend<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("prepend", args)
    }


    pub fn difference<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("difference", args)
    }


    pub fn set_insert<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("set_insert", args)
    }


    pub fn set_union<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("set_union", args)
    }


    pub fn set_intersection<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("set_intersection", args)
    }


    pub fn set_difference<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("set_difference", args)
    }


    pub fn bracket<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("bracket", args)
    }


    pub fn get_field<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("get_field", args)
    }


    pub fn has_fields<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("has_fields", args)
    }


    pub fn insert_at<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("insert_at", args)
    }


    pub fn splice_at<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("splice_at", args)
    }


    pub fn delete_at<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("delete_at", args)
    }


    pub fn change_at<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("change_at", args)
    }


    pub fn keys(&self) -> Client {
        cmd("keys")
    }


    pub fn values(&self) -> Client {
        cmd("values")
    }


    pub fn literal(&self) -> Client {
        cmd("literal")
    }


    pub fn object<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("object", args)
    }


    pub fn match_<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("match_", args)
    }


    pub fn split(&self) -> Client {
        cmd("split")
    }


    pub fn upcase(&self) -> Client {
        cmd("upcase")
    }


    pub fn downcase(&self) -> Client {
        cmd("downcase")
    }


    pub fn add<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("add", args)
    }


    pub fn sub<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("sub", args)
    }


    pub fn mul<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("mul", args)
    }


    pub fn div<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("div", args)
    }


    pub fn mod_<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("mod_", args)
    }


    pub fn and<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("and", args)
    }


    pub fn or<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("or", args)
    }


    pub fn eq<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("eq", args)
    }


    pub fn ne<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("ne", args)
    }


    pub fn gt<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("gt", args)
    }


    pub fn ge<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("ge", args)
    }


    pub fn lt<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("lt", args)
    }


    pub fn le<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("le", args)
    }


    pub fn not(&self) -> Client {
        cmd("not")
    }


    pub fn random(&self) -> Client {
        cmd("random")
    }


    pub fn round(&self) -> Client {
        cmd("round")
    }


    pub fn ceil(&self) -> Client {
        cmd("ceil")
    }


    pub fn floor(&self) -> Client {
        cmd("floor")
    }


    pub fn now(&self) -> Client {
        cmd("now")
    }


    pub fn time<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("time", args)
    }


    pub fn epoch_time<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("epoch_time", args)
    }


    pub fn iso8601<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("iso8601", args)
    }


    pub fn in_timezone<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("in_timezone", args)
    }


    pub fn timezone(&self) -> Client {
        cmd("timezone")
    }


    pub fn during<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("during", args)
    }


    pub fn date(&self) -> Client {
        cmd("date")
    }


    pub fn time_of_day(&self) -> Client {
        cmd("time_of_day")
    }


    pub fn year(&self) -> Client {
        cmd("year")
    }


    pub fn month(&self) -> Client {
        cmd("month")
    }


    pub fn day(&self) -> Client {
        cmd("day")
    }


    pub fn day_of_week(&self) -> Client {
        cmd("day_of_week")
    }


    pub fn day_of_year(&self) -> Client {
        cmd("day_of_year")
    }


    pub fn hours(&self) -> Client {
        cmd("hours")
    }


    pub fn minutes(&self) -> Client {
        cmd("minutes")
    }


    pub fn seconds(&self) -> Client {
        cmd("seconds")
    }


    pub fn to_iso8601<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("to_iso8601", args)
    }


    pub fn to_epoch_time(&self) -> Client {
        cmd("to_epoch_time")
    }


    pub fn array<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("array", args)
    }


    pub fn hashmap<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("hashmap", args)
    }


    pub fn args<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("args", args)
    }


    pub fn binary<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("binary", args)
    }


    pub fn do_<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("do_", args)
    }


    pub fn branch<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("branch", args)
    }


    pub fn for_each<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("for_each", args)
    }


    pub fn range(&self) -> Client {
        cmd("range")
    }


    pub fn error<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("error", args)
    }


    pub fn default<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("default", args)
    }


    pub fn expr<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("expr", args)
    }


    pub fn js<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("js", args)
    }


    pub fn coerce_to<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("coerce_to", args)
    }


    pub fn type_of(&self) -> Client {
        cmd("type_of")
    }


    pub fn info(&self) -> Client {
        cmd("info")
    }


    pub fn json<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("json", args)
    }


    pub fn to_json<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("to_json", args)
    }


    pub fn http<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("http", args)
    }


    pub fn uuid(&self) -> Client {
        cmd("uuid")
    }


    pub fn circle<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("circle", args)
    }


    pub fn distance<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("distance", args)
    }


    pub fn fill(&self) -> Client {
        cmd("fill")
    }


    pub fn geojson<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("geojson", args)
    }


    pub fn to_geojson(&self) -> Client {
        cmd("to_geojson")
    }


    pub fn get_intersecting<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("get_intersecting", args)
    }


    pub fn get_nearest<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("get_nearest", args)
    }


    pub fn includes<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("includes", args)
    }


    pub fn intersects<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("intersects", args)
    }


    pub fn line<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("line", args)
    }


    pub fn point<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("point", args)
    }


    pub fn polygon<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("polygon", args)
    }


    pub fn polygon_sub<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("polygon_sub", args)
    }


    pub fn grant(&self) -> Client {
        cmd("grant")
    }


    pub fn config(&self) -> Client {
        cmd("config")
    }


    pub fn rebalance(&self) -> Client {
        cmd("rebalance")
    }


    pub fn reconfigure(&self) -> Client {
        cmd("reconfigure")
    }


    pub fn status(&self) -> Client {
        cmd("status")
    }


    pub fn wait(&self) -> Client {
        cmd("wait")
    }

}
