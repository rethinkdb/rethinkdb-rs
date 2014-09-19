---
layout: api-command
language: JavaScript
permalink: api/javascript/between/
command: between
io:
    -   - table
        - selection
related_commands:
    get: get/
    getAll: get_all/
---

# Command syntax #

{% apibody %}
table.between(lowerKey, upperKey[, {index:'id', leftBound:'closed', rightBound:'open'}])
    &rarr; selection
{% endapibody %}

# Description #

Get all documents between two keys. Accepts three optional arguments: `index`,
`leftBound`, and `rightBound`. If `index` is set to the name of a secondary index,
`between` will return all documents where that index's value is in the specified range
(it uses the primary key by default). `leftBound` or `rightBound` may be set to `open`
or `closed` to indicate whether or not to include that endpoint of the range (by default,
`leftBound` is closed and `rightBound` is open).

__Example:__ Find all users with primary key >= 10 and < 20 (a normal half-open interval).

```js
r.table('marvel').between(10, 20).run(conn, callback)
```

__Example:__ Find all users with primary key >= 10 and <= 20 (an interval closed on both sides).

```js
r.table('marvel').between(10, 20, {'rightBound':'closed'}).run(conn, callback)
```


__Example:__ Find all users with primary key < 20. (You can use `null` to mean "unbounded" for either endpoint.)

```js
r.table('marvel').between(null, 20, {'rightBound':'closed'}).run(conn, callback)
```

__Example:__ Between can be used on secondary indexes too. Just pass an optional index argument giving the secondary index to query.

```js
r.table('dc').between('dark_knight', 'man_of_steel', {index:'code_name'}).run(conn, callback)
```

__Note:__ Between works with secondary indexes on date fields, but will not work with unindexed date fields. To test whether a date value is between two other dates, use the [during](/api/ruby/during) command, not `between`.

Secondary indexes can be used in extremely powerful ways with `between` and other commands; read the full article on [secondary indexes](/docs/secondary-indexes) for examples using boolean operations, `contains` and more.
