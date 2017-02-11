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
sequence.count([value | predicate_function]) &rarr; number
binary.count() &rarr; number
string.count() &rarr; number
object.count() &rarr; number
r.count(sequence | binary | string | object[, predicate_function]) &rarr; number
{% endapibody %}

# Description #

Counts the number of elements in a sequence or key/value pairs in an object, or returns the size of a string or binary object.

When `count` is called on a sequence with a predicate value or function, it returns the number of elements in the sequence equal to that value or where the function returns `True`. On a [binary](/api/python/binary) object, `count` returns the size of the object in bytes; on strings, `count` returns the string's length. This is determined by counting the number of Unicode codepoints in the string, counting combining codepoints separately.

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

__Example:__ Return the length of a Unicode string.

```py
> r.expr(u'こんにちは').count().run(conn)
5
```
