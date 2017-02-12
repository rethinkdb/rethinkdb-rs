---
layout: api-command
language: Java
permalink: api/java/group/
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
sequence.group([field | function...,]) &rarr; grouped_stream
r.group(sequence, [field | function...,]) &rarr; grouped_stream
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/group.png" class="api_command_illustration" />

# Description #

Takes a stream and partitions it into multiple groups based on the
fields or functions provided.

Two options are available via [optArg](/api/java/optarg): `index` can be the name of an index to group on (in place of a field. The `multi` flag, a boolean (default `false`), allows single documents to be assigned to multiple groups, similar to the behavior of [multi-indexes](/docs/secondary-indexes/). When `multi` is `true` and the grouping value is an array, documents will be placed in each group that corresponds to the elements of the array. If the array is empty the row will be ignored.

The data returned by `group` will be a `List<GroupedResult>`:

```java
public class GroupedResult<G,V> {
    public final G group;
    public final List<V> values;

    public GroupedResult(G group, List<V> values){
        this.group = group;
        this.values = values;
    }
}
```

Suppose that the table `games` has the following data:

```json
[
    {"id": 2, "player": "Bob", "points": 15, "type": "ranked"},
    {"id": 5, "player": "Alice", "points": 7, "type": "free"},
    {"id": 11, "player": "Bob", "points": 10, "type": "free"},
    {"id": 12, "player": "Alice", "points": 2, "type": "free"}
]
```

__Example:__ Group games by player.

```java
r.table("games").group("player").run(conn);
```

To show the returned data, we'll use JSON representation again, with `group` and `values` as the fields corresponding to the elements in each `GroupedResult`.

```json
[
    {
        "group": "Alice",
        "values": [
            {"id": 5, "player": "Alice", "points": 7, "type": "free"},
            {"id": 12, "player": "Alice", "points": 2, "type": "free"}
        ]
    },
    {
        "group": "Bob",
        "values": [
            {"id": 2, "player": "Bob", "points": 15, "type": "ranked"},
            {"id": 11, "player": "Bob", "points": 10, "type": "free"}
        ]
    }
]
```

<!-- stop -->

Commands chained after `group` will be called on each of these grouped
sub-streams, producing grouped data.

__Example:__ What is each player's best game?

```java
r.table("games").group("player").max("points").run(conn);
```

```json
[
    {
        "group": "Alice",
        "values": {"id": 5, "player": "Alice", "points": 7, "type": "free"}
    },
    {
        "group": "Bob",
        "values": {"id": 2, "player": "Bob", "points": 15, "type": "ranked"}
    }
]
```

Commands chained onto grouped data will operate on each grouped datum,
producing more grouped data.

__Example:__ What is the maximum number of points scored by each player?

```java
r.table("games").group("player").max("points").g("points").run(conn);
```

```json
[
    {
        "group": "Alice",
        "values": 7
    },
    {
        "group": "Bob",
        "values": 15
    }
]
```

You can also group by more than one field.

__Example:__ What is the maximum number of points scored by each
player for each game type?

```java
r.table("games").group("player", "type").max("points").g("points").run(conn);
```

```json
[
    {
        "group": ["Alice", "free"],
        "values": 7
    }
    {
        "group": ["Bob", "free"],
        "values": 10,
    },
    {
        "group": ["Bob", "ranked"],
        "values": 15,
    }
]
```

You can also group by a function.

__Example:__ What is the maximum number of points scored by each
player for each game type?


```java
r.table("games").group(
    game -> game.pluck("player", "type")
).max("points").g("points").run(conn);
```

```json
[
    {
        "group": {"player": "Alice", "type": "free"},
        "values": 7
    },
    {
        "group": {"player": "Bob", "type": "free"},
        "values": 10
    },
    {
        "group": {"player": "Bob", "type": "ranked"},
        "values": 15
    }
]
```

Using a function, you can also group by date on a ReQL [date field](/docs/dates-and-times/javascript/).

__Example:__ How many matches have been played this year by month?

```java
r.table("matches").group(
    match -> r.array(match.g("date").year(), match.g("date").month())
).count().run(conn);
```

```json
[
    {
        "group": [2014, 2],
        "values": 2
    },
    {
        "group": [2014, 3],
        "values": 2
    },
    {
        "group": [2014, 4],
        "values": 1
    },
    {
        "group": [2014, 5],
        "values": 3
    }
]
```

You can also group on an index (primary key or secondary).

__Example:__ What is the maximum number of points scored by game type?


```java
r.table("games").group().optArg("index", "type")
 .max("points").g("points").run(conn);
```

```json
[
    {
        "group": "free",
        "values": 10
    },
    {
        "group": "ranked",
        "values": 15
    }
]
```

# Organizing by value with **multi** #

Suppose that the table `games2` has the following data:

```json
[
    { "id": 1, "matches": {"a": [1, 2, 3], "b": [4, 5, 6]} },
    { "id": 2, "matches": {"b": [100], "c": [7, 8, 9]} },
    { "id": 3, "matches": {"a": [10, 20], "c": [70, 80]} }
]
```

Using the `multi` option we can group data by match A, B or C.

```java
r.table("games2").group(
    row -> row.g("matches").keys()
).optArg("multi", true).run(conn);
```

```json
[
    {
        "group": "a",
        "values": [ <id 1>, <id 3> ]
    },
    {
        "group": "b",
        "values": [ <id 1>, <id 2> ]
    },
    {
        "group": "c",
        "values": [ <id 2>, <id 3> ]
    }
]
```

(The full result set is abbreviated in the figure; `<id 1>, <id 2>` and `<id 3>` would be the entire documents matching those keys.)

__Example:__ Use [map](/api/java/map) and [sum](/api/java/sum) to get the total points scored for each match.

```java
r.table("games2").group(
    row -> row.g("matches").keys()
).optArg("multi", true).ungroup().map(
    doc -> r.hashMap("match", doc.g("group")).with(
        "total", doc.g("reduction").sum(
            set -> set.g("matches").bracket(doc.g("group")).sum()
        )
    )
).run(conn);
```

```json
[
    { "match": "a", "total": 36 },
    { "match": "b", "total": 115 },
    { "match": "c", "total": 174 }
]
```

The inner `sum` adds the scores by match within each document; the outer `sum` adds those results together for a total across all the documents.

# Ungrouping #

If you want to operate on all the groups rather than operating on each
group (e.g. if you want to order the groups by their reduction), you
can use [ungroup](/api/java/ungroup/) to turn a grouped stream or
grouped data into an array of objects representing the groups.

The format of the array returned by `ungroup` is the same as the
default native format of grouped data in the JavaScript driver and
Data Explorer.

__Example:__ Ungrouping grouped data.

```java
r.table('games').group('player').max('points')['points'].ungroup().run(conn);
```

```json
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

```java
r.table("games").group("player").max("points").g("points").ungroup()
 .orderBy(r.desc("reduction")).run(conn);
```

```json
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
[times](/docs/dates-and-times/).)  In Java, grouped data are
transformed into an `List`.  If you instead want to receive the raw
pseudotype from the server, you can specify `group_format: "raw"` as an optional
argument to `run`:

__Example:__ Get back the raw `GROUPED_DATA` pseudotype.

```java
r.table("games").group("player").avg("points").run(conn).optArg("group_format", "raw");
```

```json
{
    "$reql_type$": "GROUPED_DATA",
    "data": [
        ["Alice", 4.5],
        ["Bob", 12.5]
    ]
}
```

You might also want to use the [ungroup](/api/java/ungroup/)
command (see above), which will turn the grouped data into an array of
objects on the server.


# Performance Details #

If you run a query that returns a grouped stream, it will be
automatically converted to grouped data before being sent back to you
(there is currently no efficient way to stream groups from RethinkDB).
This grouped data is subject to the array size limit, by default 100,000 elements (see [run](/api/java/run) for details on how to use the `array_limit` argument to change this).

In general, operations on grouped streams will be efficiently
distributed, and operations on grouped data won't be.  You can figure
out what you're working with by putting `typeOf` on the end of your
query.  Below are efficient and inefficient examples.

__Example:__ Efficient operation.

```java
// r.table("games").group("player").typeOf().run(conn);
// Returns "GROUPED_STREAM"
r.table("games").group("player").min("points").run(conn); // EFFICIENT
```

__Example:__ Inefficient operation.

```java
// r.table("games").group("player").orderBy("score").typeOf().run(conn);
// Returns "GROUPED_DATA"
r.table("games").group("player").orderBy("score").nth(0).run(conn); // INEFFICIENT
```

What does it mean to be inefficient here?  When operating on grouped
data rather than a grouped stream, *all* of the data has to be
available on the node processing the query.  This means that the
operation will only use one server's resources, and will require
memory proportional to the size of the grouped data it's operating
on.  (In the case of the [orderBy](/api/java/order_by/) in the inefficient example, that
means memory proportional **to the size of the table**.)  The array
limit is also enforced for grouped data, so the `orderBy` example
would fail for tables with more than 100,000 rows without changing the `arrayLimit` option to `run`.

# More Examples #

__Example:__ What is the maximum number of points scored by each
player in free games?

```java
r.table("games").filter(
    game -> game.g("type").eq("free")
).group("player").max("points").g("points").run(conn);
```

```json
[
    {
        "group": "Alice",
        "values": 7
    },
    {
        "group": "Bob",
        "values": 10
    }
]
```

__Example:__ What is each player's highest even and odd score?

```java
r.table("games").group(
    "name", game -> game.g("points").mod(2)
).max("points").g("points").run(conn);
```

```json
[
    {
        "group": ["Alice", 1],
        "values": 7
    },
    {
        "group": ["Bob", 0],
        "values": 10
    },
    {
        "group": ["Bob", 1],
        "values": 15
    }
]
```
