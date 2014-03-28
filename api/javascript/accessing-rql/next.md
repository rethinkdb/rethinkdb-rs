---
layout: api-command
language: JavaScript
permalink: api/javascript/next/
command: next
rb: false
py: false
io:
    -   - cursor
        - undefined
related_commands:
    hasNext: has_next/
    each: each/
    toArray: to_array/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.next(callback)
array.next(callback)
{% endapibody %}

# Description #

Get the next element in the cursor.

Calling `next` the first time on a cursor provides the first element of the cursor.

__Example:__ Let's grab the next element!

```js
cursor.next(function(err, row) {
    if (err) throw err;
    processRow(row);
});
```

__Note:__ The canonical way to retrieve all the results is to use [each](../each/)
or [toArray](../toArray/). The `next` command should be used only when you may not
retrieve all the elements of a cursor or want to delay some operations.



__Example:__ You can retrieve all the elements of a cursor with the `next`
command using recursion.

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

__Example:__ Another equivalent way to retrieve all the documents is to catch a `RqlDriverError`.

```js
query.run( conn, function(err, cursor) {
    if (err) throw err;

    var fetchNext = function(err, result) {
        if (err) {
            if (((err.name === "RqlDriverError") && err.message === "No more rows in the cursor.")) {
                console.log("No more data to process")
                // If you use one connection per query, the connection should be closed here.
                // conn.close()
            }
            else {
                throw err;
            }
        }
        else {
            processRow(result);
            cursor.next(fetchNext);
        }
    }
    cursor.next(fetchNext);
})
```

__Example:__ With `next`, not all results have to be retrieved from a cursor
-- to stop retrieving results, break out of the recursive function. For example, this
recursive function will stop retrieving results when the `checkRow` function returns true:

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
