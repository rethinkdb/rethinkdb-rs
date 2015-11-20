---
layout: api-command
language: Python
permalink: api/python/between/
command: between
alias:
    - api/python/minval/
    - api/python/maxval/
related_commands:
    get: get/
    get_all: get_all/
    filter: filter/
---

# Command syntax #

{% apibody %}
table.between(lower_key, upper_key[, options]) &rarr; table_slice
table_slice.between(lower_key, upper_key[, options]) &rarr; table_slice
{% endapibody %}

# Description #

Get all documents between two keys. Accepts three optional arguments: `index`, `left_bound`, and `right_bound`. If `index` is set to the name of a secondary index, `between` will return all documents where that index's value is in the specified range (it uses the primary key by default). `left_bound` or `right_bound` may be set to `open` or `closed` to indicate whether or not to include that endpoint of the range (by default, `left_bound` is closed and `right_bound` is open).

You may also use the special constants `r.minval` and `r.maxval` for boundaries, which represent "less than any index key" and "more than any index key" respectively. For instance, if you use `r.minval` as the lower key, then `between` will return all documents whose primary keys (or indexes) are less than the specified upper key.

If you use arrays as indexes (compound indexes), they will be sorted using [lexicographical order][lo]. Take the following range as an example:

	[[1, "c"] ... [5, "e"]]

This range includes all compound keys:

* whose first item is 1 and second item is equal or greater than "c";
* whose first item is between 1 and 5, *regardless of the value of the second item*;
* whose first item is 5 and second item is less than or equal to "e".

[lo]: https://en.wikipedia.org/wiki/Lexicographical_order

__Example:__ Find all users with primary key >= 10 and < 20 (a normal half-open interval).

```py
r.table('marvel').between(10, 20).run(conn)
```

__Example:__ Find all users with primary key >= 10 and <= 20 (an interval closed on both sides).

```py
r.table('marvel').between(10, 20, right_bound='closed').run(conn)
```

__Example:__ Find all users with primary key < 20.

```py
r.table('marvel').between(r.minval, 20).run(conn)
```

__Example:__ Find all users with primary key > 10.

```py
r.table('marvel').between(10, r.maxval, left_bound='open').run(conn)
```

__Example:__ Between can be used on secondary indexes too. Just pass an optional index argument giving the secondary index to query.

```py
r.table('dc').between('dark_knight', 'man_of_steel', index='code_name').run(conn)
```

__Example:__ Get all users whose full name is between "John Smith" and "Wade Welles."

```py
r.table("users").between(["Smith", "John"], ["Welles", "Wade"],
    index="full_name").run(conn)
```

__Example:__ Get the top 10 ranked teams in order.

```py
r.table("teams").order_by(index="rank").between(1, 11).run(conn)
```

__Note:__ When `between` is chained after [order_by](/api/python/order_by), both commands must use the same index; `between` will default to the index `order_by` is using, so in this example `"rank"` is automatically being used by `between`. Trying to specify another index will result in a `ReqlRuntimeError`.

__Example:__ Subscribe to a [changefeed](/docs/changefeeds/python) of teams ranked in the top 10.

```py
changes = r.table("teams").between(1, 11, index="rank").changes().run(conn)
```

{% infobox %}
Between works with secondary indexes on date fields, but will not work with unindexed date fields. To test whether a date value is between two other dates, use the [during](/api/python/during) command, not `between`.

Secondary indexes can be used in extremely powerful ways with `between` and other commands; read the full article on [secondary indexes](/docs/secondary-indexes) for examples using boolean operations, `contains` and more.

RethinkDB uses byte-wise ordering for `between` and does not support Unicode collations; non-ASCII characters will be sorted by UTF-8 codepoint.
{% endinfobox %}
