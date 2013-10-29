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

__Example:__ You can retrieve all the elements of a cursor with the `next` and `hasNext`
commands using recursion.

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

__Example:__ If you want to retrieve elements as long as a function (let's call it
`checkRow`) returns `true`, you just need to break the recursion.


```js
query.run( conn, function(err, cursor) {
    if (err) throw err;

    var fetchNext = function(err, result) {
        if (err) throw err;

        processRow(result);

        if (checkRow(result)) {
            if (cursor.hasNext()) {
                cursor.next(fetchNext);
            }
            // If you use one connection per query, the connection should be closed.
            // else { conn.close() }
        }
        else {
            cursor.close()
            // If you use one connection per query, the connection should be closed here.
            // else { conn.close() }
        }
    }

    if (cursor.hasNext()) {
        cursor.next(fetchNext);
    }
    // If you use one connection per query, the connection should be closed.
    // else { conn.close() }

})
```

__Note:__ If you don't retrieve all the documents in a cursor and don't need it anymore,
you should manually close it. If you don't, the driver will keep some results in memory
which will result in a memory leak.


