---
layout: api-command 
language: JavaScript
permalink: api/javascript/coerce_to/
command: coerceTo
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/javascript/control-structures/coerceTo.md
io:
    -   - sequence
        - array
    -   - value
        - string
    -   - array
        - object
    -   - object
        - array
---

{% apibody %}
sequence.coerceTo(typeName) → array
value.coerceTo(typeName) → string
array.coerceTo(typeName) → object
object.coerceTo(typeName) → array
{% endapibody %}

Converts a value of one type into another. 

You can convert: a selection, sequence, or object into an ARRAY, an array of pairs into an OBJECT, and any DATUM into a STRING.

__Example:__ Convert a table to an array.

```js
r.table('marvel').coerceTo('array').run(conn, callback)
```


__Example:__ Convert an array of pairs into an object.


```js
r.expr([['name', 'Ironman'], ['victories', 2000]]).coerceTo('object').run(conn, callback)
```

__Example:__ Convert a number to a string.

```js
r.expr(1).coerceTo('string').run(conn, callback)
```

