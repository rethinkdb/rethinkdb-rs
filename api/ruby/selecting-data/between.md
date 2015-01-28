---
layout: api-command
language: Ruby
permalink: api/ruby/between/
command: between
related_commands:
    get: get/
    get_all: get_all/
    filter: filter/
---

# Command syntax #

{% apibody %}
table.between(lower_key, upper_key
    [, :index => 'id', :left_bound => 'closed', :right_bound => 'open'])
        &rarr; selection
{% endapibody %}

# Description #

Get all documents between two keys. Accepts three optional arguments: `index`, `left_bound`, and `right_bound`. If `index` is set to the name of a secondary index, `between` will return all documents where that index's value is in the specified range (it uses the primary key by default). `left_bound` or `right_bound` may be set to `open` or `closed` to indicate whether or not to include that endpoint of the range (by default, `left_bound` is closed and `right_bound` is open).

Note that compound indexes are sorted using [lexicographical order][lo]. Take the following range as an example:

	[[1, "c"] ... [5, "e"]]

This range includes all compound keys:

* whose first item is 1 and second item is equal or greater than "c";
* whose first item is between 1 and 5, *regardless of the value of the second item*;
* whose first item is 5 and second item is less than or equal to "e".

[lo]: https://en.wikipedia.org/wiki/Lexicographical_order

__Example:__ Find all users with primary key >= 10 and < 20 (a normal half-open interval).

```rb
r.table('marvel').between(10, 20).run(conn)
```

__Example:__ Find all users with primary key >= 10 and <= 20 (an interval closed on both sides).

```rb
r.table('marvel').between(10, 20, :right_bound => 'closed').run(conn)
```


__Example:__ Find all users with primary key < 20. (You can use `nil` to mean "unbounded" for either endpoint.)

```rb
r.table('marvel').between(nil, 20, :right_bound => 'closed').run(conn)
```

__Example:__ Between can be used on secondary indexes too. Just pass an optional index argument giving the secondary index to query.

```rb
r.table('dc').between('dark_knight', 'man_of_steel', :index => 'code_name').run(conn)
```

__Example:__ Get all users whose full name is between "John Smith" and "Wade Welles."

```rb
r.table("users").between(["Smith", "John"], ["Welles", "Wade"],
    :index => "full_name").run(conn)
```


__Example:__ Subscribe to a [changefeed](/docs/changefeeds/javascript) of teams ranked in the top 10.

```rb
changes = r.table("teams").between(1, 11, :index => "rank").changes().run(conn)
```

__Note:__ Between works with secondary indexes on date fields, but will not work with unindexed date fields. To test whether a date value is between two other dates, use the [during](/api/ruby/during) command, not `between`.

Secondary indexes can be used in extremely powerful ways with `between` and other commands; read the full article on [secondary indexes](/docs/secondary-indexes) for examples using boolean operations, `contains` and more.
