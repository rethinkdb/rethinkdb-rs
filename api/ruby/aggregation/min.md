---
layout: api-command
language: Ruby
permalink: api/ruby/min/
command: min
related_commands:
    map: map/
    reduce: reduce/
    count: count/
    sum: sum/
    avg: avg/
    max: max/
    group: group/
---

# Command syntax #

{% apibody %}
sequence.min([field_or_function]) &rarr; element
{% endapibody %}

# Description #

Finds the minimum of a sequence.  If called with a field name, finds
the element of that sequence with the smallest value in that field.
If called with a function, calls that function on every element of the
sequence and returns the element of that sequence which produced the
smallest value, ignoring any elements where the function returns
`null` or produces a non-existence error.

Produces a non-existence error when called on an empty sequence.  You
can handle this case with `default`.

__Example:__ What's the minimum of 3, 5, and 7?

```rb
r([3, 5, 7]).min().run(conn)
```

__Example:__ Which user has scored the fewest points?

```rb
r.table('users).min('points').run(conn)
```

__Example:__ Which user has scored the fewest points, counting bonus points?

```rb
r.table('users).min{|user| user['points'] + user['bonus_points']}.run(conn)
```

__Example:__ What is the smallest number of points any user has ever scored?

```rb
r.table('users).min('points')['points'].run(conn)
```

__Example:__ Which user has scored the fewest points?  (But return
`nil` instead of erroring if no users have ever scored points.)

```rb
r.table('users').min('points').default(nil).run(conn)
```
