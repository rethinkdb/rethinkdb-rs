---
layout: api-command
language: JavaScript
permalink: api/javascript/indexes_of/
command: indexesOf
io:
    -   - sequence
        - array
---

# Command syntax #

{% apibody %}
sequence.indexesOf(datum | predicate) &rarr; array
{% endapibody %}

# Description #

Get the indexes of an element in a sequence. If the argument is a predicate, get the indexes of all elements matching it.

__Example:__ Find the position of the letter 'c'.

```js
r.expr(['a','b','c']).indexesOf('c').run(conn, callback)
```

__Example:__ Find the popularity ranking of invisible heroes.

```js
r.table('marvel').union(r.table('dc')).orderBy('popularity').indexesOf(
    r.row('superpowers').contains('invisibility')
).run(conn, callback)
```
