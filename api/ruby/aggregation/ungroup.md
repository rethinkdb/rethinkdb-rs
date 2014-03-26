---
layout: api-command
language: Ruby
permalink: api/ruby/ungroup/
command: ungroup
related_commands:
    group: group/
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
data explorer.

__Example:__ What is the maximum number of points scored by each
player, with the highest scorers first?

Suppose that the table `games` has the following data:

```rb
[
    {"id" => 2, "player" => "Bob", "points" => 15, "type" => "ranked"},
    {"id" => 5, "player" => "Alice", "points" => 7, "type" => "free"},
    {"id" => 11, "player" => "Bob", "points" => 10, "type" => "free"},
    {"id" => 12, "player" => "Alice", "points" => 2, "type" => "free"}
]
```

We can use this query:

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

__Example:__ Select one random player and all their games.

```rb
r.table('games').group('player').ungroup().sample(1).run(conn)
```

Result:

```rb
[
    {
        "group" => "Bob",
        "reduction" => [
            {"id" => 2, "player" => "Bob", "points" => 15, "type" => "ranked"},
            {"id" => 11, "player" => "Bob", "points" => 10, "type" => "free"}
        ]
    }
]
```

Note that if you didn't call `ungroup`, you would instead select one
random game from each player:

```rb
r.table('games').group('player').sample(1).run(conn)
```

Result:

```rb
{
    "Alice" => [
        {"id" => 5, "player" => "Alice", "points" => 7, "type" => "free"}
    ],
    "Bob" => [
        {"id" => 11, "player" => "Bob", "points" => 10, "type" => "free"}
    ]
}
```

__Example:__ Types!

```rb
r.table('games').group('player').type_of().run(conn) # Returns "GROUPED_STREAM"
r.table('games').group('player').ungroup().type_of().run(conn) # Returns "ARRAY"
r.table('games').group('player').avg('points').run(conn) # Returns "GROUPED_DATA"
r.table('games').group('player').avg('points').ungroup().run(conn) #Returns "ARRAY"
```
