---
layout: api-command
language: Ruby
permalink: api/ruby/group
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
sequence.group(field_or_function...) &rarr; grouped_stream
{% endapibody %}

# Description #

`group` takes a stream and partitions it into multiple groups based on
the fields or functions provided.  Commands chained after `group` will
be called on each of these grouped sub-streams.  Chaining a reduction
on the end (`reduce`, `sum`, etc.) will result in grouped data, which
behaves similarly.

When grouped data are returned to the client, they are transformed
into a client-specific native type (much like times).  In Ruby, they
are transformed into a `Hash`.  If you instead want to receive the raw
pseudotype from the server (e.g. if you're planning to serialize the
result as JSON), you can specify `group_format: 'raw'` as an optional
argument to `run`:

```rb
> r.table('games').group('player).avg('points').run(conn)
{"Alice"=>3, "Bob"=>9, ...}
> r.table('games').group('player').avg('points').run(conn, group_format:'raw')
{"$reql_type$"=>"GROUPED_DATA", "data"=>[["Alice", 3], ["Bob", 9], ...]}
```

If you want to operate on all the groups rather than operating on each
group (e.g. if you want to order the groups by their reduction), you
can use `ungroup` to turn a grouped stream or grouped data into an
array of objects representing the groups.

```rb
> r.table('games').group('player).avg('points').run(conn)
{"Alice"=>3, "Bob"=>9, ...}
> r.table('games').group('player).avg('points').ungroup().run(conn)
[{"group"=>"Alice", "reduction"=>3}, {"group"=>"Bob", "reduction"=>9}, ...]
```

__Example:__ What is each player's best game?

```rb
> r.table('games').group('player').max('points').run(conn)
{"Alice"=>{"id"=>5, "player"=>"Alice", "points"=>7, "type"=>"free"},
 "Bob"=>{"id"=>2, "player"=>"Bob", "points"=>15, "type"=>"ranked"},
 ...}
```

__Example:__ What is the maximum number of points scored by each player?

```rb
> r.table('games').group('player').max('points')['points'].run(conn)
{"Alice"=>7, "Bob"=>15, ...}
```

__Example:__ What is the maximum number of points scored by each
player for each game type?

```rb
> r.table('games').group('player', 'type').max('points')['points'].run(conn)
{["Alice", "free"]=>7,
 ["Alice", "ranked"]=>1,
 ["Bob", "free"]=>11,
 ["Bob", "ranked"]=>15,
 ...}
> r.table('games') \
   .group{|game| game.pluck('player', 'type')} \
   .max('points')['points'].run(conn)
{{"player"=>"Alice", "type"=>"free"}=>7,
 {"player"=>"Alice", "type"=>"ranked}=>1,
 {"player"=>"Bob, "type"=>"free"}=>11,
 {"player"=>Bob, "type"=>"ranked}=>15,
 ...}
```

__Example:__ What is the maximum number of points scored by each
player in free games?

```rb
> r.table('games').filter{|game| game['type'].eq('free')}
   .group('player').max('points')['points'].run(conn)
{"Alice"=>7, "Bob"=>11, ...}
```

__Example:__ What is each player's highest even and odd score?

```rb
> r.table('games') \
   .group('name', lambda {|game| game['points'] % 2}) \
   .max('points')['points'].run(conn)
{["Alice", 0]=>4,
 ["Alice", 1]=>7,
 ["Bob", 0]=>12,
 ["Bob", 1]=>15,
 ...}
```

__Example:__ What is the maximum number of points scored by each
player, with the highest scorers first?

```rb
> r.table('games') \
   .group('player').max('points')['points'] \
   .ungroup().order_by(r.desc('reduction')).run(conn)
[{"group"=>"Bob", "reduction"=>15}, {"group"=>"Alice", "reduction"=>7}, ...]
```

