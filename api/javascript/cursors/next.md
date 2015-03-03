---
layout: api-command
language: JavaScript
permalink: api/javascript/next/
command: next
io:
    -   - cursor
        - undefined
related_commands:
    each: each/
    toArray: to_array/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.next(callback)
array.next(callback)
cursor.next() &rarr; promise
array.next() &rarr; promise
{% endapibody %}

# Description #

Get the next element in the cursor.

Calling `next` the first time on a cursor provides the first element of the cursor. If the data set is exhausted (e.g., you have retrieved all the documents in a table), a `RqlDriverError` error will be passed to the callback when `next` is called.

__Example:__ Retrieve the next element.

```js
cursor.next(function(err, row) {
    if (err) throw err;
    processRow(row);
});
```

__Note:__ The canonical way to retrieve all the results is to use [each](../each/) or [toArray](../to_array/). The `next` command should be used only when you may not retrieve all the elements of a cursor or want to delay some operations.

__Example:__ You can retrieve all the elements of a cursor with the `next`
command using recursion.

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
            if (checkRow(result)) {
                cursor.next(fetchNext);
            }
            else {
                cursor.close()
                // If you use one connection per query, the connection should be closed here.
                // conn.close()
            }
        }
    }
    cursor.next(fetchNext);
})
```

__Example:__ You can retrieve all the elements of a cursor with the `next`
command using recursion and promises.

```js
query.run(conn).then(function(cursor) {
    var errorHandler = function(err) {
        if (((err.name === "RqlDriverError") && err.message === "No more rows in the cursor.")) {
            console.log("No more data to process")
            // If you use one connection per query, the connection should be closed here.
            // conn.close()
        }
        else {
            throw err;
        }
    }
    var fetchNext = function(result) {
        processRow(result);
        cursor.next().then(fetchNext).error(errorHandler);
    }

    cursor.next().then(fetchNext).error(errorHandler);
}).error(function(err) {
    throw err;
});
```
