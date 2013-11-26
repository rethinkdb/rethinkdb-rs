---
layout: api-command
language: Python
permalink: api/python/coerce_to/
command: coerce_to
---

# Command syntax #

{% apibody %}
sequence.coerce_to(type_name) &rarr; array
value.coerce_to(type_name) &rarr; string
array.coerce_to(type_name) &rarr; object
object.coerce_to(type_name) &rarr; array
{% endapibody %}

# Description #

Converts a value of one type into another.

You can convert: a selection, sequence, or object into an ARRAY, an array of pairs into an OBJECT, and any DATUM into a STRING.

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

