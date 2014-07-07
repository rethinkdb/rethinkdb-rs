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

Get all documents between two keys. Accepts three optional arguments: `index`,
`left_bound`, and `right_bound`. If `index` is set to the name of a secondary index,
`between` will return all documents where that index's value is in the specified range
(it uses the primary key by default). `left_bound` or `right_bound` may be set to `open`
or `closed` to indicate whether or not to include that endpoint of the range (by default,
`left_bound` is closed and `right_bound` is open).

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

__Note:__ To filter all documents between *times* (and dates), do not use `between`. Instead, using the [during](/api/ruby/during) command.