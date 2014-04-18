---
layout: api-command
language: JavaScript
permalink: api/javascript/order_by/
command: orderBy
io:
    -   - sequence
        - stream
    -   - array
        - array
related_commands:
    table: table/
---

# Command syntax #

{% apibody %}
table.orderBy([key1...], {index: index_name}) &rarr; selection<stream>
selection.orderBy(key1, [key2...]) &rarr; selection<array>
sequence.orderBy(key1, [key2...]) &rarr; array
{% endapibody %}

# Description #

Sort the sequence by document values of the given key(s). `orderBy` defaults to ascending
ordering. To explicitly specify the ordering, wrap the attribute with either `r.asc` or
`r.desc`.

__Example:__ Order our heroes by a series of performance metrics.

```js
r.table('marvel').orderBy('enemiesVanquished', 'damselsSaved').run(conn, callback)
```

__Example:__ Indexes can be used to perform more efficient orderings. Notice that the index ordering always has highest precedence. Thus the following example is equivalent to the one above.

```js
r.table('marvel').orderBy('damselsSaved', {index: 'enemiesVanquished'}).run(conn, callback)
```

__Example:__ You can also specify a descending order when using an index.

```js
r.table('marvel').orderBy({index: r.desc('enemiesVanquished')}).run(conn, callback)
```

__Example:__ Let's lead with our best vanquishers by specify descending ordering.

```js
r.table('marvel').orderBy(r.desc('enemiesVanquished'), r.asc('damselsSaved'))
.run(conn, callback)
```

__Example:__ You can use a function for ordering instead of just selecting an attribute.

```js
r.table('marvel').orderBy(function (doc) { return doc('enemiesVanquished') + doc('damselsSaved'); }).run(conn, callback)
```

__Example:__ Functions can also be used descendingly.

```js
r.table('marvel').orderBy(r.desc(function (doc) { return doc('enemiesVanquished') + doc('damselsSaved'); })).run(conn, callback)
```

