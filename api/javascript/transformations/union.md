---
layout: api-command
language: JavaScript
permalink: api/javascript/union/
command: union
io:
    -   - sequence
        - array
---

# Command syntax #

{% apibody %}
stream.union(sequence[, sequence, ...]) &rarr; stream
array.union(sequence[, sequence, ...]) &rarr; array
{% endapibody %}

# Description #

Merge two or more sequences. (Note that ordering is not guaranteed by `union`.)

__Example:__ Construct a stream of all heroes.

```js
r.table('marvel').union(r.table('dc')).run(conn, callback);
```

__Example:__ Combine four arrays into one.

```js
r.expr([1, 2]).union([3, 4], [5, 6], [7, 8, 9]).run(conn, callback);
// Result passed to callback
[1, 2, 3, 4, 5, 6, 7, 8, 9]
```

__Example:__ Combine four arrays into one.

```js
r.expr([1, 2]).union([3, 4], [5, 6], [7, 8, 9]).run(conn, callback)
// Result passed to callback
[1, 2, 3, 4, 5, 6, 7, 8, 9]
```

__Example:__ Create a [changefeed][cf] from the first example.

```js
r.table('marvel').union(r.table('dc')).changes().run(conn, callback);
```

Now, when any heroes are added, modified or deleted from either table, a change notification will be sent out.

[cf]: /docs/changefeeds/javascript
