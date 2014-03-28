---
layout: api-command
language: Python
permalink: api/python/count/
command: count
related_commands:
    map: map/
    reduce: reduce/
    sum: sum/
    avg: avg/
    min: min/
    max: max/
    group: group/
---

# Command syntax #

{% apibody %}
sequence.count([value_or_predicate]) &rarr; number
{% endapibody %}

# Description #

Counts the number of elements in a sequence.  If called with a value,
counts the number of times that value occurs in the sequence.  If
called with a predicate function, counts the number of elements in the
sequence where that function returns `True`.


__Example:__ Count the number of users.

```py
r.table('users').count().run(conn)
```

__Example:__ Count the number of 18 year old users.

```py
r.table('users')['age'].count(18).run(conn)
```

__Example:__ Count the number of users over 18.

```py
r.table('users')['age'].count(lambda age: age > 18).run(conn)
```

```py
r.table('users').count(lambda user: user['age'] > 18).run(conn)
```
