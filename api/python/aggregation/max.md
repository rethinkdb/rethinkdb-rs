---
layout: api-command
language: Python
permalink: api/python/max/
command: max
related_commands:
    map: map/
    reduce: reduce/
    count: count/
    sum: sum/
    avg: avg/
    min: min/
    group: group/
---

# Command syntax #

{% apibody %}
sequence.max([field_or_function]) &rarr; element
{% endapibody %}

# Description #

Finds the maximum of a sequence.  If called with a field name, finds
the element of that sequence with the largest value in that field.  If
called with a function, calls that function on every element of the
sequence and returns the element which produced the largest value,
ignoring any elements where the function returns `None` or produces a
non-existence error.

Produces a non-existence error when called on an empty sequence.  You
can handle this case with `default`.

__Example:__ What's the maximum of 3, 5, and 7?

```py
r.expr([3, 5, 7]).max().run(conn)
```

__Example:__ Which user has scored the most points?

```py
r.table('users').max('points').run(conn)
```

__Example:__ Which user has scored the most points, counting bonus points?

```py
r.table('users').max(lambda user:
    user['points'] + user['bonus_points']
).run(conn)
```

__Example:__ What is the largest number of points any user has ever scored?

```py
r.table('users').max('points')['points'].run(conn)
```

__Example:__ Which user has scored the most points?  (But return
`None` instead of erroring if no users have ever scored points.)

```py
r.table('users').max('points').default(None).run(conn)
```
