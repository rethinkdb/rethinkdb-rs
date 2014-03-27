---
layout: api-command
language: Ruby
permalink: api/ruby/sum/
command: sum
related_commands:
    map: map/
    reduce: reduce/
    count: count/
    avg: avg/
    min: min/
    max: max/
    group: group/
---

# Command syntax #

{% apibody %}
sequence.sum([field_or_function]) &rarr; number
{% endapibody %}

# Description #

Sums all the elements of a sequence.  If called with a field name,
sums all the values of that field in the sequence, skipping elements
of the sequence that lack that field.  If called with a function,
calls that function on every element of the sequence and sums the
results, skipping elements of the sequence where that function returns
`nil` or a non-existence error.

Returns `0` when called on an empty sequence.

__Example:__ What's 3 + 5 + 7?

```rb
r([3, 5, 7]).sum().run(conn)
```

__Example:__ How many points have been scored across all games?

```rb
r.table('games').sum('points').run(conn)
```

__Example:__ How many points have been scored across all games,
counting bonus points?

```rb
r.table('games').sum{|game| game['points'] + game['bonus_points']}.run(conn)
```
