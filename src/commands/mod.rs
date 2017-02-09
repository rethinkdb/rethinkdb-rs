//! The command reference

commands! {
    db_list,            changes,            db_create,          db_drop,            table_create,       table_drop,
    table_list,         index_create,       index_drop,         index_list,         index_rename,       index_status,
    index_wait,         insert,             update,             replace,            delete,             sync,
    db,                 table,              get,                get_all,            between,            filter,
    inner_join,         outer_join,         eq_join,            zip,                map,                with_fields,
    concat_map,         order_by,           skip,               limit,              slice,              nth,
    offsets_of,         is_empty,           union,              sample,             group,              ungroup,
    reduce,             fold,               count,              sum,                avg,                min,
    max,                distinct,           contains,           pluck,              without,            merge,
    append,             prepend,            difference,         set_insert,         set_union,          set_intersection,
    set_difference,     get_field,          has_fields,         insert_at,          splice_at,          delete_at,
    change_at,          keys,               values,             literal,            expr,               match_,
    split,              upcase,             downcase,           add,                sub,                mul,
    div,                mod_,               and,                or,                 eq,                 ne,
    gt,                 ge,                 lt,                 le,                 not,                random,
    round,              ceil,               floor,              now,                time,               epoch_time,
    iso8601,            in_timezone,        timezone,           during,             date,               time_of_day,
    year,               month,              day,                day_of_week,        day_of_year,        hours,
    minutes,            seconds,            to_iso8601,         to_epoch_time,      wait,               binary,
    do_,                branch,             for_each,           range,              error,              default,
    status,             js,                 coerce_to,          type_of,            info,               json,
    to_json,            http,               uuid,               circle,             distance,           fill,
    geojson,            to_geojson,         get_intersecting,   get_nearest,        includes,           intersects,
    line,               point,              polygon,            polygon_sub,        grant,              config,
    rebalance,          reconfigure,        with_args,
}

mod args;
#[cfg(feature = "with_io")]
mod io;
#[cfg(feature = "with_io")]
pub use self::io::*;
