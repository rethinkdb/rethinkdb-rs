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

__Example:__ Another similar way to retrieve all the docunents is to look for a
`RqlDriverError`

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
        else if (cursor.hasNext()) {
            processRow(result);
            cursor.next(fetchNext);
        }
    }
    cursor.next(fetchNext);
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

Note: You should not have to worry about getting a `maximum call stack exceed` error.
The stack will be reset every time the cursor fetches data. In case of an array, 
the driver will call `setImmediate` every 100 rows.
