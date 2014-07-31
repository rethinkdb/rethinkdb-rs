---
layout: api-command
language: JavaScript
permalink: api/javascript/group/
command: group
io:
    -   - sequence
        - grouped_stream
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
sequence.group(fieldOrFunction..., [{index: "indexName"}]) &rarr; grouped_stream
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/group.png" class="api_command_illustration" />

# Description #

Takes a stream and partitions it into multiple groups based on the
fields or functions provided.

__Example:__ Grouping games by player.

Suppose that the table `games` has the following data:

```js
[
    {id: 2, player: "Bob", points: 15, type: "ranked"},
    {id: 5, player: "Alice", points: 7, type: "free"},
    {id: 11, player: "Bob", points: 10, type: "free"},
    {id: 12, player: "Alice", points: 2, type: "free"}
]
```

Grouping games by player can be done with:

```js
> r.table('games').group('player').run(conn, callback)

// Result passed to callback
[
    {
        group: "Alice",
        reduction: [
            {id: 5, player: "Alice", points: 7, type: "free"},
            {id: 12, player: "Alice", points: 2, type: "free"}
        ]
    },
    {
        group: "Bob",
        reduction: [
            {id: 2, player: "Bob", points: 15, type: "ranked"},
            {id: 11, player: "Bob", points: 10, type: "free"}
        ]
    }
]
```


Commands chained after `group` will be called on each of these grouped
sub-streams, producing grouped data.

__Example:__ What is each player's best game?

```js
> r.table('games').group('player').max('points').run(conn, callback)

// Result passed to callback
[
    {
        group: "Alice",
        reduction: {id: 5, player: "Alice", points: 7, type: "free"}
    },
    {
        group: "Bob",
        reduction: {id: 2, player: "Bob", points: 15, type: "ranked"}
    }
]
```

Commands chained onto grouped data will operate on each grouped datum,
producing more grouped data.

__Example:__ What is the maximum number of points scored by each player?

```js
> r.table('games').group('player').max('points')('points').run(conn, callback)

// Result passed to callback
[
    {
        group: "Alice",
        reduction: 7
    },
    {
        group: "Bob",
        reduction: 15
    }
]
```

You can also group by more than one field.

__Example:__ What is the maximum number of points scored by each
player for each game type?

```js
> r.table('games').group('player', 'type').max('points')('points').run(conn, callback)

// Result passed to callback
[
    {
        group: ["Alice", "free"],
        reduction: 7
    }
    {
        group: ["Bob", "free"],
        reduction: 10,
    },
    {
        group: ["Bob", "ranked"],
        reduction: 15,
    }
]
```

You can also group by a function.

__Example:__ What is the maximum number of points scored by each
player for each game type?


```js
> r.table('games')
    .group(function(game) {
        return game.pluck('player', 'type')
    }).max('points')('points').run(conn, callback)

// Result passed to callback
[
    {
        group: {"player": "Alice", "type": "free"},
        reduction: 7
    },
    {
        group: {"player": "Bob", "type": "free"}
        reduction: 10
    },
    {
        group: {"player": "Bob", "type": "ranked"},
        reduction: 15
    }
]
```

You can also group by an index.

__Example:__ What is the maximum number of points scored by game type?


```js
> r.table('games').group({index:'type'}).max('points')('points').run(conn, callback)

// Result passed to callback
[
    {
        group: "free",
        reduction: 10
    },
    {
        group: "ranked",
        reduction: 15
    }
]
```

If you want to operate on all the groups rather than operating on each
group (e.g. if you want to order the groups by their reduction), you
can use [ungroup](/api/javascript/ungroup/) to turn a grouped stream or
grouped data into an array of objects representing the groups.

__Example:__ Ungrouping grouped data.

```js
> r.table('games').group('player').max('points')('points').ungroup().run(conn, callback)

// Result passed to callback
[
    {
        group: "Alice",
        reduction: 7
    },
    {
        group: "Bob",
        reduction: 15
    }
]
```

Ungrouping is useful e.g. for ordering grouped data, or for inserting
grouped data into a table.

__Example:__ What is the maximum number of points scored by each
player, with the highest scorers first?

```js
> r.table('games')
   .group('player').max('points')('points')
   .ungroup().orderBy(r.desc('reduction')).run(conn, callback)

// Result passed to callback
[
    {
        group: "Bob",
        reduction: 15
    },
    {
        group: "Alice",
        reduction: 7
    }
]
```


# Implementation Details #

When grouped data are returned to the client, they are transformed
into a client-specific native type.  (Something similar is done with
[times](/docs/dates-and-times/).)  In JavaScript, grouped data are
transformed into an `Array`.  If you instead want to receive the raw
pseudotype from the server, you can specify `groupFormat: 'raw'` as an optional
argument to `run`:

__Example:__ Get back the raw `GROUPED_DATA` pseudotype.

```js
> r.table('games').group('player').avg('points').run(conn, {groupFormat:'raw'}, callback)

// Result passed to callback
{
    $reql_type$: "GROUPED_DATA",
    data: [
        ["Alice", 4.5],
        ["Bob", 12.5]
    ]
}
```

Not passing the `group_format` flag would return:

```js
[
    {
        group: "Alice":
        reduction: 4.5
    },
    {
        group: "Bob"
        reduction: 12.5
    }
]
```


You might also want to use the [ungroup](/api/javascript/ungroup/)
command (see above), which will turn the grouped data into an array of
objects on the server.


# Performance Details #

If you run a query that returns a grouped stream, it will be
automatically converted to grouped data before being sent back to you
(there is currently no efficient way to stream groups from RethinkDB).
This grouped data is subject to the array size limit (which means you
can't have more than 100,000 groups or 100,000 rows in the output).

In general, operations on grouped streams will be efficiently
distributed, and operations on grouped data won't be.  You can figure
out what you're working with by putting `typeOf` on the end of your
query.  Below are efficient and inefficient examples.

__Example:__ Efficient operation.

```js
// r.table('games').group('player').typeOf().run(conn, callback)
// Returns "GROUPED_STREAM"
r.table('games').group('player').min('points').run(conn, callback) // EFFICIENT
```

__Example:__ Inefficient operation.

```js
// r.table('games').group('player').orderBy('score').typeOf().run(conn, callback)
// Returns "GROUPED_DATA"
r.table('games').group('player').orderBy('score').nth(0).run(conn, callback) // INEFFICIENT
```

What does it mean to be inefficient here?  When operating on grouped
data rather than a grouped stream, *all* of the data has to be
available on the node processing the query.  This means that the
operation will only use one machine's resources, and will require
memory proportional to the size of the grouped data it's operating
on.  (In the case of the `orderBy` in the inefficient example, that
means memory proportional **to the size of the table**.)  The array
limit is also enforced for grouped data, so the `orderBy` example
would fail for tables with more than 100,000 rows.

# More Examples #

__Example:__ What is the maximum number of points scored by each
player in free games?

```js
> r.table('games').filter( r.row('type').eq('free'))
    .group('player').max('points')('points')
    .run(conn, callback)

// Result passed to callback
[
    {
        group: "Alice":
        reduction: 7
    },
    {
        group: "Bob",
        reduction: 10
    }
]
```

__Example:__ What is each player's highest even and odd score?

```js
r.table('games')
    .group('name', function(game) {
        return game('points').mod(2)
    }).max('points')('points').run(conn, callback)

// Result passed to callback
[
    {
        group: ["Alice", 1]
        reduction: 7,
    },
    {
        group: ["Bob", 0],
        reduction: 10
    },
    {
        group: ["Bob", 1],
        reduction: 15
    }
]
```
