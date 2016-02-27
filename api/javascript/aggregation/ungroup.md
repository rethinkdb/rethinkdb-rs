---
layout: api-command
language: JavaScript
permalink: api/javascript/ungroup/
command: ungroup
related_commands:
    group: group/
io:
    -   - grouped_stream
        - array
---

# Command syntax #

{% apibody %}
grouped_stream.ungroup() &rarr; array
grouped_data.ungroup() &rarr; array
{% endapibody %}

# Description #

Takes a grouped stream or grouped data and turns it into an array of
objects representing the groups.  Any commands chained after `ungroup`
will operate on this array, rather than operating on each group
individually.  This is useful if you want to e.g. order the groups by
the value of their reduction.

The format of the array returned by `ungroup` is the same as the
default native format of grouped data in the javascript driver and
data explorer.

Suppose that the table `games` has the following data:

```js
[
    {id: 2, player: "Bob", points: 15, type: "ranked"},
    {id: 5, player: "Alice", points: 7, type: "free"},
    {id: 11, player: "Bob", points: 10, type: "free"},
    {id: 12, player: "Alice", points: 2, type: "free"}
]
```

__Example:__ What is the maximum number of points scored by each
player, with the highest scorers first?

```js
r.table('games')
   .group('player').max('points')('points')
   .ungroup().orderBy(r.desc('reduction')).run(conn, callback)
```

<!-- stop -->

Result:

```js
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

__Example:__ Select one random player and all their games.

```js
r.table('games').group('player').ungroup().sample(1).run(conn, callback)
```

Result:

```js
[
    {
        group: "Bob",
        reduction: [
            {id: 2, player: "Bob", points: 15, type: "ranked"},
            {id: 11, player: "Bob", points: 10, type: "free"},

        ]
    }

]
```

Note that if you didn't call `ungroup`, you would instead select one
random game from each player:

```js
r.table('games').group('player').sample(1).run(conn)
```

Result:

```js
[
    {
        group: "Alice",
        reduction: [
            {id: 5, player: "Alice", points: 7, type: "free"}
        ]
    },
    {
        group: "Bob",
        reduction: [
            {id: 11, player: "Bob", points: 10, type: "free"}
        ]
    }
}
```

__Example:__ Finding the arithmetic mode of an array of values:
```javascript
r.expr([1,2,2,2,3,3]).group(r.row).count().ungroup().orderBy('reduction').nth(-1)('group')
```

Result:

```json
2
```

__Example:__ Types!

```js
r.table('games').group('player').typeOf().run(conn, callback) // Returns "GROUPED_STREAM"
r.table('games').group('player').ungroup().typeOf().run(conn, callback) // Returns "ARRAY"
r.table('games').group('player').avg('points').run(conn, callback) // Returns "GROUPED_DATA"
r.table('games').group('player').avg('points').ungroup().run(conn, callback) // Returns "ARRAY"
```
