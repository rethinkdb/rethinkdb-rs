---
layout: api-command 
language: JavaScript
permalink: api/javascript/has_next/
command: hasNext 
rb: false
py: false
io:
    -   - cursor
        - undefined
related_commands:
    next: next/
    each: each/
    toArray: to_array/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.hasNext() &rarr; bool
array.hasNext() &rarr; bool
{% endapibody %}

# Description #

Check if there are more elements in the cursor.

__Example:__ Are there more elements in the cursor?

```js
var hasMore = cursor.hasNext();
```

__Example:__ Retrieve all the elements of a cursor using the `next` and `hasNext`
commands and recursion.

```js
query.run( conn, function(err, cursor) {
    if (err) throw err;

    var fetchNext = function(err, result) {
        if (err) throw err;
        if (cursor.hasNext()) {
            processRow(result);
            cursor.next(fetchNext);
        }
        // If you use one connection per query, the connection should be closed.
        // else { conn.close() }
    }

    if (cursor.hasNext()) {
        cursor.next(fetchNext);
    }
    // If you use one connection per query, the connection should be closed.
    // else { conn.close() }
})
```
