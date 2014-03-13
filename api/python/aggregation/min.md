---
layout: api-command
language: Python
permalink: api/python/min/
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
sequence and returns the element which produced the smallest value,
ignoring any elements where the function returns `null` or produces a
non-existence error.

Produces a non-existence error when called on an empty sequence.  You
can handle this case with `default`.

__Example:__ What's the minimum of 3, 5, and 7?

```py
r.expr([3, 5, 7]).min().run(conn)
```

__Example:__ Which user has scored the fewest points?

```py
r.table('users').min('points').run(conn)
```

__Example:__ Which user has scored the fewest points, counting bonus points?

```py
r.table('users').min(lambda user:
    user['points'] + user['bonus_points']
).run(conn)
```

__Example:__ What is the smallest number of points any user has ever scored?

```py
r.table('users').min('points')['points'].run(conn)
```

__Example:__ Which user has scored the fewest points?  (But return
`None` instead of erroring if no users have ever scored points.)

```py
r.table('users').min('points').default(None).run(conn)
```
