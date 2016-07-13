---
layout: api-command
language: Python
permalink: api/python/group/
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
sequence.group(field | function..., [index=<indexname>, multi=False]) &rarr; grouped_stream
r.group(sequence, field | function..., [index=<indexname>, multi=False]) &rarr; grouped_stream
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/group.png" class="api_command_illustration" />

# Description #

Takes a stream and partitions it into multiple groups based on the
fields or functions provided.

With the `multi` flag single documents can be assigned to multiple groups, similar to the behavior of [multi-indexes](/docs/secondary-indexes/python). When `multi` is `True` and the grouping value is an array, documents will be placed in each group that corresponds to the elements of the array. If the array is empty the row will be ignored.

Suppose that the table `games` has the following data:

```py
[
    {"id": 2, "player": "Bob", "points": 15, "type": "ranked"},
    {"id": 5, "player": "Alice", "points": 7, "type": "free"},
    {"id": 11, "player": "Bob", "points": 10, "type": "free"},
    {"id": 12, "player": "Alice", "points": 2, "type": "free"}
]
```


__Example:__ Group games by player.

```py
> r.table('games').group('player').run(conn)

{
    "Alice": [
        {"id": 5, "player": "Alice", "points": 7, "type": "free"},
        {"id": 12, "player": "Alice", "points": 2, "type": "free"}
    ],
    "Bob": [
        {"id": 2, "player": "Bob", "points": 15, "type": "ranked"},
        {"id": 11, "player": "Bob", "points": 10, "type": "free"}
    ]
}
```

<!-- stop -->

Commands chained after `group` will be called on each of these grouped
sub-streams, producing grouped data.

__Example:__ What is each player's best game?

```py
> r.table('games').group('player').max('points').run(conn)

{
    "Alice": {"id": 5, "player": "Alice", "points": 7, "type": "free"},
    "Bob": {"id": 2, "player": "Bob", "points": 15, "type": "ranked"}
}
```

Commands chained onto grouped data will operate on each grouped datum,
producing more grouped data.

__Example:__ What is the maximum number of points scored by each player?

```py
> r.table('games').group('player').max('points')['points'].run(conn)

{
    "Alice": 7,
    "Bob": 15
}
```

You can also group by more than one field.

__Example:__ What is the maximum number of points scored by each
player for each game type?

```py
> r.table('games').group('player', 'type').max('points')['points'].run(conn)

{
    ("Alice", "free"): 7,
    ("Bob", "free"): 10,
    ("Bob", "ranked"): 15
}
```

You can also group by a function.

__Example:__ What is the maximum number of points scored by each
player for each game type?

```py
> r.table('games')
    .group(lambda game:
        game.pluck('player', 'type')
    ).max('points')['points'].run(conn)

{
    frozenset([('player', 'Alice'), ('type', 'free')]): 7,
    frozenset([('player', 'Bob'), ('type', 'free')]): 10,
    frozenset([('player', 'Bob'), ('type', 'ranked')]): 15,
}
```

Using a function, you can also group by date on a ReQL [date field](/docs/dates-and-times/javascript/).

__Example:__ How many matches have been played this year by month?

```py
> r.table('matches').group(
      lambda match: [match['date'].year(), match['date'].month()]
  ).count().run(conn)

{
    (2014, 2): 2,
    (2014, 3): 2,
    (2014, 4): 1,
    (2014, 5): 3
}
```

You can also group on an index (primary key or secondary).

__Example:__ What is the maximum number of points scored by game type?

```py
> r.table('games').group(index='type').max('points')['points'].run(conn)

{
    "free": 10,
    "ranked": 15
}
```

# Organizing by value with **multi** #

Suppose that the table `games2` has the following data:

```py
[
    { 'id': 1, 'matches': {'a': [1, 2, 3], 'b': [4, 5, 6]} },
    { 'id': 2, 'matches': {'b': [100], 'c': [7, 8, 9]} },
    { 'id': 3, 'matches': {'a': [10, 20], 'c': [70, 80]} }
]
```

Using the `multi` option we can group data by match A, B or C.

```py
> r.table('games2').group(r.row['matches'].keys(), multi=True).run(conn)

[
    {
        'group': 'a',
        'reduction': [ <id 1>, <id 3> ]
    },
    {
        'group': 'b',
        'reduction': [ <id 1>, <id 2> ]
    },
    {
        'group': 'c',
        'reduction': [ <id 2>, <id 3> ]
    }
]
```

(The full result set is abbreviated in the figure; `<id 1>, <id 2>` and `<id 3>` would be the entire documents matching those keys.)

__Example:__ Use [map](/api/python/map) and [sum](/api/python/sum) to get the total points scored for each match.

```py
r.table('games2').group(r.row['matches'].keys(), multi=True).ungroup().map(
    lambda doc: { 'match': doc['group'], 'total': doc['reduction'].sum(
        lambda set: set['matches'][doc['group']].sum()
    )}).run(conn)

[
    { 'match': 'a', 'total': 36 },
    { 'match': 'b', 'total': 115 },
    { 'match': 'c', 'total': 174 }
]
```

The inner `sum` adds the scores by match within each document; the outer `sum` adds those results together for a total across all the documents.

# Ungrouping #

If you want to operate on all the groups rather than operating on each
group (e.g. if you want to order the groups by their reduction), you
can use [ungroup](/api/python/ungroup/) to turn a grouped stream or
grouped data into an array of objects representing the groups.

__Example:__ Ungrouping grouped data.

```py
> r.table('games').group('player').max('points')['points'].ungroup().run(conn)

[
    {
        "group": "Alice",
        "reduction": 7
    },
    {
        "group": "Bob",
        "reduction": 15
    }
]
```

Ungrouping is useful e.g. for ordering grouped data, or for inserting
grouped data into a table.

__Example:__ What is the maximum number of points scored by each
player, with the highest scorers first?

```py
> r.table('games').group('player').max('points')['points'].ungroup().order_by(
        r.desc('reduction')).run(conn)

[
    {
        "group": "Bob",
        "reduction": 15
    },
    {
        "group": "Alice",
        "reduction": 7
    }
]
```


# Implementation Details #

When grouped data are returned to the client, they are transformed
into a client-specific native type.  (Something similar is done with
[times](/docs/dates-and-times/).)  In Python, grouped data are
transformed into a `dictionary`. If the group value is an `array`, the
key is converted to a `tuple`. If the group value is a `dictionary`,
it will be converted to a `frozenset`.

If you instead want to receive the raw
pseudotype from the server (e.g. if you're planning to serialize the
result as JSON), you can specify `group_format: 'raw'` as an optional
argument to `run`:

__Example:__ Get back the raw `GROUPED_DATA` pseudotype.

```py
> r.table('games').group('player').avg('points').run(conn, group_format='raw')

{
    "$reql_type$": "GROUPED_DATA",
    "data": [
        ["Alice", 4.5],
        ["Bob", 12.5]
    ]
}
```

Not passing the `group_format` flag would return:

```py
{
    "Alice": 4.5,
    "Bob": 12.5
}
```



You might also want to use the [ungroup](/api/python/ungroup/)
command (see above), which will turn the grouped data into an array of
objects on the server.

# Performance Details #

If you run a query that returns a grouped stream, it will be
automatically converted to grouped data before being sent back to you
(there is currently no efficient way to stream groups from RethinkDB).
This grouped data is subject to the array size limit (see [run](/api/python/run)).

In general, operations on grouped streams will be efficiently
distributed, and operations on grouped data won't be.  You can figure
out what you're working with by putting `type_of` on the end of your
query.  Below are efficient and inefficient examples.

__Example:__ Efficient operation.

```py
# r.table('games').group('player').type_of().run(conn)
# Returns "GROUPED_STREAM"
r.table('games').group('player').min('points').run(conn) # EFFICIENT
```

__Example:__ Inefficient operation.

```py
# r.table('games').group('player').order_by('score').type_of().run(conn)
# Returns "GROUPED_DATA"
r.table('games').group('player').order_by('score').nth(0).run(conn) # INEFFICIENT
```

What does it mean to be inefficient here?  When operating on grouped
data rather than a grouped stream, *all* of the data has to be
available on the node processing the query.  This means that the
operation will only use one server's resources, and will require
memory proportional to the size of the grouped data it's operating
on.  (In the case of the [order_by](/api/python/order_by/) in the inefficient example, that
means memory proportional **to the size of the table**.)  The array
limit is also enforced for grouped data, so the `order_by` example
would fail for tables with more than 100,000 rows unless you used the `array_limit` option with `run`.

# More Examples #

__Example:__ What is the maximum number of points scored by each
player in free games?

```py
> r.table('games').filter(lambda game:
        game['type'] = 'free'
    ).group('player').max('points')['points'].run(conn)

{
    "Alice": 7,
    "Bob": 10
}
```

__Example:__ What is each player's highest even and odd score?

```py
> r.table('games')
    .group('name', lambda game:
        game['points'] % 2
    ).max('points')['points'].run(conn)

{
    ("Alice", 1): 7,
    ("Bob", 0): 10,
    ("Bob", 1): 15
}
```
