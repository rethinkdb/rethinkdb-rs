---
layout: api-command
language: Java
permalink: api/java/ungroup/
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
default native format of grouped data in the JavaScript driver and
Data Explorer.

Suppose that the table `games` has the following data:

```json
[
    {"id": 2, "player": "Bob", "points": 15, "type": "ranked"},
    {"id": 5, "player": "Alice", "points": 7, "type": "free"},
    {"id": 11, "player": "Bob", "points": 10, "type": "free"},
    {"id": 12, "player": "Alice", "points": 2, "type": "free"}
]
```

__Example:__ What is the maximum number of points scored by each
player, with the highest scorers first?

```java
r.table("games").group("player").max("points").g("points").ungroup()
 .orderBy(r.desc("reduction")).run(conn);
```

<!-- stop -->

The result:

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

__Example:__ Select one random player and all their games.

```java
r.table("games").group("player").ungroup().sample(1).run(conn);
```

Result:

```json
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
```

Note that if you didn't call `ungroup`, you would instead select one
random game from each player:

```java
r.table("games").group("player").sample(1).run(conn);
```

Result: (Note this is a JSON representation of a `List<GroupedResult>`; see the [group](/api/java/group) documentation for more details.)

```json
[
    {
        "group": "Alice",
        "values": [
            {"id": 5, "player": "Alice", "points": 7, "type": "free"}
        ]
    },
    {
        "group": "Bob",
        "values": [
            {"id": 11, "player": "Bob", "points": 10, "type": "free"}
        ]
    }
}
```



__Example:__ Types!

```java
r.table('games').group('player').typeOf().run(conn); // Returns "GROUPED_STREAM"
r.table('games').group('player').ungroup().typeOf().run(conn); // Returns "ARRAY"
r.table('games').group('player').avg('points').run(conn); // Returns "GROUPED_DATA"
r.table('games').group('player').avg('points').ungroup().run(conn); // Returns "ARRAY"
```
