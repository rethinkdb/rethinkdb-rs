---
layout: api-command
language: Python
permalink: api/python/row/
command: row
rb: false
java: ralse
---

# Command syntax #

{% apibody %}
r.row &rarr; value
{% endapibody %}

# Description #

Returns the currently visited document.

{% infobox %}
Note that `row` does not work within subqueries to access nested documents; you should use anonymous functions to access those documents instead. (See the last example.)
{% endinfobox %}

__Example:__ Get all users whose age is greater than 5.

```py
r.table('users').filter(r.row['age'] > 5).run(conn)
```


__Example:__ Access the attribute 'child' of an embedded document.

```py
r.table('users').filter(r.row['embedded_doc']['child'] > 5).run(conn)
```


__Example:__ Add 1 to every element of an array.

```py
r.expr([1, 2, 3]).map(r.row + 1).run(conn)
```


__Example:__ For nested queries, use functions instead of `row`.

```py
r.table('users').filter(
    lambda doc: doc['name'] == r.table('prizes').get('winner')
).run(conn)
```

