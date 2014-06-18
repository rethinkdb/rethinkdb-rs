---
layout: api-command
language: Python
permalink: api/python/coerce_to/
command: coerce_to
---

# Command syntax #

{% apibody %}
sequence.coerce_to('array') &rarr; array
value.coerce_to('string') &rarr; string
string.coerce_to('number') &rarr; number
array.coerce_to('object') &rarr; object
object.coerce_to('array') &rarr; array
{% endapibody %}

# Description #

Converts a value of one type into another.

* a sequence, selection or object can be coerced to an array
* an array of key-value pairs can be coerced to an object
* a string can be coerced to a number
* any datum (single value) can be converted to a string

__Example:__ Convert a table to an array.

```py
r.table('marvel').coerce_to('array').run(conn)
```

__Example:__ Convert an array of pairs into an object.

```py
r.expr([['name', 'Ironman'], ['victories', 2000]]).coerce_to('object').run(conn)
```


__Example:__ Convert a number to a string.

```py
r.expr(1).coerce_to('string').run(conn)
```

