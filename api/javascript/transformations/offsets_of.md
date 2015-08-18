---
layout: api-command
language: JavaScript
permalink: api/javascript/offsets_of/
command: offsetsOf
alias: api/javascript/indexes_of/
io:
    -   - sequence
        - array
---

# Command syntax #

{% apibody %}
sequence.offsetsOf(datum | predicate_function) &rarr; array
{% endapibody %}

# Description #

Get the indexes of an element in a sequence. If the argument is a predicate, get the indexes of all elements matching it.

__Example:__ Find the position of the letter 'c'.

```js
r.expr(['a','b','c']).offsetsOf('c').run(conn, callback)
```

__Example:__ Find the popularity ranking of invisible heroes.

```js
r.table('marvel').union(r.table('dc')).orderBy('popularity').offsetsOf(
    r.row('superpowers').contains('invisibility')
).run(conn, callback)
```
