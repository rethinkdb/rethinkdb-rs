---
layout: api-command
language: Ruby
permalink: api/ruby/count/
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
sequence where that function returns `true`.

__Example:__ Count the number of users.

```rb
r.table('users').count().run(conn)
```

__Example:__ Count the number of 18 year old users.

```rb
r.table('users')['age'].count(18).run(conn)
```

__Example:__ Count the number of users over 18.

```rb
r.table('users')['age'].count{|age| age > 18}.run(conn)
```

```rb
r.table('users').count{|user| user['age'] > 18}.run(conn)
```
