---
layout: api-command
language: Ruby
permalink: api/ruby/group/
command: group
related_commands:
    ungroup: ungroup/
    map: map/
    reduce: reduce/
    count: count/
    sum: sum/
    avg: avg/
    min: min/
    max: max/
---

# Command syntax #

{% apibody %}
sequence.group(field_or_function..., [:index => 'index_name']) &rarr; grouped_stream
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/group.png" class="api_command_illustration" />

# Description #

Takes a stream and partitions it into multiple groups based on the
fields or functions provided.

__Example:__ Grouping games by player.

Suppose that the table `games` has the following data:

```rb
[
    {"id" => 2, "player" => "Bob", "points" => 15, "type" => "ranked"},
    {"id" => 5, "player" => "Alice", "points" => 7, "type" => "free"},
    {"id" => 11, "player" => "Bob", "points" => 10, "type" => "free"},
    {"id" => 12, "player" => "Alice", "points" => 2, "type" => "free"}
]
```

Grouping games by player can be done with:

```rb
r.table('games').group('player').run(conn)
```

Result:

```rb
{
    "Alice" => [
        {"id" => 5, "player" => "Alice", "points" => 7, "type" => "free"},
        {"id" => 12, "player" => "Alice", "points" => 2, "type" => "free"}
    ],
    "Bob" => [
        {"id" => 2, "player" => "Bob", "points" => 15, "type" => "ranked"},
        {"id" => 11, "player" => "Bob", "points" => 10, "type" => "free"}
    ]
}
```

Commands chained after `group` will be called on each of these grouped
sub-streams, producing grouped data.

__Example:__ What is each player's best game?

```rb
r.table('games').group('player').max('points').run(conn)
```

Result:

```rb
{
    "Alice" => {"id" => 5, "player" => "Alice", "points" => 7, "type" => "free"},
    "Bob" => {"id" => 2, "player" => "Bob", "points" => 15, "type" => "ranked"}
}
```

Commands chained onto grouped data will operate on each grouped datum,
producing more grouped data.

__Example:__ What is the maximum number of points scored by each player?

```rb
r.table('games').group('player').max('points')['points'].run(conn)
```

Result:

```rb
{
    "Alice" => 7,
    "Bob" => 15
}
```

You can also group by more than one field.

__Example:__ What is the maximum number of points scored by each
player for each game type?

```rb
r.table('games').group('player', 'type').max('points')['points'].run(conn)
```

Result:

```rb
{
    ["Alice", "free"] => 7,
    ["Bob", "free"] => 10,
    ["Bob", "ranked"] => 15,
    ...
}
```


You can also group by a function.

__Example:__ What is the maximum number of points scored by each
player for each game type?

```rb
r.table('games')
    .group{|game| game.pluck('player', 'type')}
    .max('points')['points'].run(conn)
```

Result:

```rb
{
    {"player" => "Alice", "type" => "free"} => 7,
    {"player" => "Bob", "type" => "free"} => 10,
    {"player" => "Bob", "type" => "ranked"} => 15
}
```

You can also group by an index.

__Example:__ What is the maximum number of points scored by game type?


```rb
r.table('games').group(:index => 'type').max('points')['points'].run(conn)
```

Result:

```rb
{
    "free" => 10,
    "ranked" => 15
}
```

If you want to operate on all the groups rather than operating on each
group (e.g. if you want to order the groups by their reduction), you
can use [ungroup](/api/ruby/ungroup/) to turn a grouped stream or
grouped data into an array of objects representing the groups.

__Example:__ Ungrouping grouped data.

```rb
r.table('games').group('player').max('points')['points'].ungroup().run(conn)
```

Result:

```rb
[
    {
        "group" => "Alice",
        "reduction" => 7
    },
    {
        "group" => "Bob",
        "reduction" => 15
    }
]
```

Ungrouping is useful e.g. for ordering grouped data, or for inserting
grouped data into a table.

__Example:__ What is the maximum number of points scored by each
player, with the highest scorers first?

```rb
r.table('games')
   .group('player').max('points')['points']
   .ungroup().order_by(r.desc('reduction')).run(conn)
```

Result:

```rb
[
    {
        "group" => "Bob",
        "reduction" => 15
    },
    {
        "group" => "Alice",
        "reduction" => 7
    }
]
```


# Implementation Details #

When grouped data are returned to the client, they are transformed
into a client-specific native type.  (Something similar is done with
[times](/docs/dates-and-times/).)  In Ruby, grouped data are
transformed into a `Hash`.  If you instead want to receive the raw
pseudotype from the server (e.g. if you're planning to serialize the
result as JSON), you can specify `group_format: 'raw'` as an optional
argument to `run`:

__Example:__ Get back the raw `GROUPED_DATA` pseudotype.


```rb
r.table('games').group('player').avg('points').run(conn, group_format:'raw')
```

```rb
{
    "$reql_type$" => "GROUPED_DATA",
    "data" => [
        ["Alice", 4.5],
        ["Bob", 12.5]
    ]
}
```

Not passing the `group_format` flag would return:

```rb
{
    "Alice" => 4.5,
    "Bob" => 12.5
}
```


You might also want to use the [ungroup](/api/ruby/ungroup/)
command (see above), which will turn the grouped data into an array of
objects on the server.


# Performance Details #

If you run a query that returns a grouped stream, it will be
automatically converted to grouped data before being sent back to you
(there is currently no efficient way to stream groups from RethinkDB).
This grouped data is subject to the array size limit, by default 100,000 elements (see [run](/api/ruby/run) for details on how to use the `array_limit` option to change this).

In general, operations on grouped streams will be efficiently
distributed, and operations on grouped data won't be.  You can figure
out what you're working with by putting `type_of` on the end of your
query.  Below are efficient and inefficient examples.

__Example:__ Efficient operation.

```rb
# r.table('games').group('player').type_of().run(conn)
# Returns "GROUPED_STREAM"
r.table('games').group('player').min('points').run(conn) # EFFICIENT
```

__Example:__ Inefficient operation.

```rb
# r.table('games').group('player').order_by('score').type_of().run(conn)
# Returns "GROUPED_DATA"
r.table('games').group('player').order_by('score').nth(0).run(conn) # INEFFICIENT
```

What does it mean to be inefficient here?  When operating on grouped
data rather than a grouped stream, *all* of the data has to be
available on the node processing the query.  This means that the
operation will only use one machine's resources, and will require
memory proportional to the size of the grouped data it's operating
on.  (In the case of the `order_by` in the inefficient example, that
means memory proportional **to the size of the table**.)  The array
limit is also enforced for grouped data, so the `order_by` example
would fail for tables with more than 100,000 rows without changing the `array_limit` option to `run`.

# More Examples #

__Example:__ What is the maximum number of points scored by each
player in free games?

```rb
r.table('games').filter{|game| game['type'].eq('free')}
   .group('player').max('points')['points'].run(conn)
```

Result:

```rb
{
    "Alice" => 7,
    "Bob" => 10
}
```

__Example:__ What is each player's highest even and odd score?

```rb
r.table('games')
   .group('name', lambda {|game| game['points'] % 2})
   .max('points')['points'].run(conn)
```

Result:

```rb
{
    ["Alice", 1] => 7,
    ["Bob", 0] => 10,
    ["Bob", 1] => 15
}
```

