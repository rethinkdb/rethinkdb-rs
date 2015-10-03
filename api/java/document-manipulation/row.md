---
layout: api-command
language: Java
permalink: api/javascript/row/
command: row
rb: false
io:
    -   - r
        - value
---

# Command syntax #

{% apibody %}
r.row &rarr; value
{% endapibody %}

# Description #

Returns the currently visited document. Note that `row` does not work within subqueries to access nested documents; you should use anonymous functions to access those documents instead. (See the last example.)

__Example:__ Get all users whose age is greater than 5.

```js
r.table('users').filter(r.row('age').gt(5)).run(conn)
```


__Example:__ Access the attribute 'child' of an embedded document.

```js
r.table('users').filter(r.row('embedded_doc')('child') > 5).run(conn)
```


__Example:__ Add 1 to every element of an array.

```js
r.expr([1, 2, 3]).map(r.row.add(1)).run(conn)
```


__Example:__ For nested queries, use functions instead of `row`.

```js
r.table('users').filter(function(doc) {
    return doc('name').eq(r.table('prizes').get('winner'))
}).run(conn)
```

