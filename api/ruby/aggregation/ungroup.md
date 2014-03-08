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

`ungroup` takes a grouped stream or grouped data and turns it into an
array of objects representing the groups.  Any commands chained after
`ungroup` will operate on this array, rather than operating on each
group individually.  This is useful if you want to e.g. order the
groups by the value of their reduction.

The format of the array returned by `ungroup` is the same as the
default native format of grouped data in the javascript driver and
data explorer.

__Example:__ What is the maximum number of points scored by each
player, with the highest scorers first?

```rb
> r.table('games') \
   .group('player').max('points')['points'] \
   .ungroup().order_by(r.desc('reduction')).run(conn)
[{"group"=>"Bob", "reduction"=>15}, {"group"=>"Alice", "reduction"=>7}, ...]
```

__Example:__ Select one random player and all their games.

```rb
> r.table('games').group('player').ungroup().sample(1).run(conn)
[{"group"=>"Bob",
  "reduction"=>
   [{"id"=>0, "player"=>"Bob", "points"=>1},
    {"id"=>2, "player"=>"Bob", "points"=>15},
    ...]}]
```

Note that if you didn't call `ungroup`, you would instead select one
random game from each player:

```rb
> r.table('games').group('player').sample(1).run(conn)
{"Alice"=>[{"id"=>5, "player"=>"Alice", "points"=>7}],
 "Bob"=>[{"id"=>2, "player"=>"Bob", "points"=>15}],
 ...}
```

__Example:__ Types!

```rb
> r.table('games').group('player').type_of().run(conn)
"GROUPED_STREAM"
> r.table('games').group('player').ungroup().type_of().run(conn)
"ARRAY"
> r.table('games').group('player').avg('points').run(conn)
"GROUPED_DATA"
> r.table('games').group('player').avg('points').ungroup().run(conn)
"ARRAY"
```
