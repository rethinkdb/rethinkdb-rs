---
layout: api-command 
language: JavaScript
permalink: api/javascript/between/
command: between
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/selecting-data/between.md
---

{% apibody %}
table.between(lowerKey, upperKey[, {index:'id', left_bound:'closed', right_bound:'open'}]) → selection
{% endapibody %}

Get all documents between two keys. Accepts three optional arguments: `index`,
`left_bound`, and `right_bound`. If `index` is set to the name of a secondary index,
`between` will return all documents where that index's value is in the specified range
(it uses the primary key by default). `left_bound` or `right_bound` may be set to `open`
or `closed` to indicate whether or not to include that endpoint of the range (by default,
`left_bound` is closed and `right_bound` is open).

__Example:__ Find all users with primary key >= 10 and < 20 (a normal half-open interval).

```js
r.table('marvel').between(10, 20).run(conn, callback)
```

__Example:__ Find all users with primary key >= 10 and <= 20 (an interval closed on both sides).

```js
r.table('marvel').between(10, 20, {'right_bound':'closed'}).run(conn, callback)
```


__Example:__ Find all users with primary key < 20. (You can use `NULL` to mean "unbounded" for either endpoint.)

```js
r.table('marvel').between(null, 20, {'right_bound':'closed'}).run(conn, callback)
```

__Example:__ Between can be used on secondary indexes too. Just pass an optional index argument giving the secondary index to query.

```js
r.table('dc').between('dark_knight', 'man_of_steel', {index:'code_name'}).run(conn, callback)
```

