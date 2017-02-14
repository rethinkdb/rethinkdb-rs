---
layout: api-command
language: JavaScript
permalink: api/javascript/each_async/
command: eachAsync
io:
    -   - cursor
        - undefined
related_commands:
    each: each/
---

# Command syntax #

{% apibody %}
sequence.eachAsync(function[, errorFunction]) &rarr; promise
{% endapibody %}

# Description #

Lazily iterate over a cursor, array, or feed one element at a time. `eachAsync` always returns a promise that will be resolved once all rows are returned.

The first, required function passed to `eachAsync` takes either one or two functions as arguments. The first is a callback to process each row as it is emitted; the second is an optional callback which will be executed when all row processing is completed.

```javascript
function(rowProcess[, final])
```

The `rowProcess` callback receives the row as its first argument; it may also take an optional second argument, which is a callback function to be executed after each row has been processed.

```javascript
function(row[, rowFinished])
```

If you accept the `rowFinished` callback, it _must_ be called at the end of each row. If you call `rowFinished` with any value, iteration will stop, and the value will be wrapped in `error.message` for the error handler.

If you do _not_ use `rowFinished`, the `rowProcess` callback can end iteration early by returning any value _other_ than a Promise. If it returns a non-Promise value (other than `undefined`), then `final` will receive an error object. If it returns a Promise, the Promise will be resolved before iteration continues; if the resolved Promise returns a value, iteration will be stopped and an error object will be passed to `final`.

If you provide a `final` callback, it will always be executed when row processing is completed (the end of the sequence is hit, iteration is stopped prematurely, or an error occurs). The `final` callback will receive an `error` object if an error is thrown or `rowProcess` returns any value (other than a Promise). If `final` returns any value it will be ignored.

To summarize all of the above in code:

```javascript
// process each row asynchronously
cursor.eachAsync(function (row) {
    doSomethingWith(row);
});

// as above, but using rowFinished callback
cursor.eachAsync(function (row, rowFinished) {
    doSomethingWith(row);
    rowFinished();
});

// as above, but using final callback
cursor.eachAsync(function (row, rowFinished) {
    doSomethingWith(row);
    rowFinished();
}, function (final) {
    // the 'final' argument will only be defined when there is an error
    console.log('Final called with:', final);
});
```

__Example:__ Process all the elements in a stream, using `then` and `catch` for handling the end of the stream and any errors. Note that iteration may be stopped in the first callback (`rowProcess`) by returning any non-Promise value.

```javascript
cursor.eachAsync(function (row) {
    var ok = processRowData(row);
    if (!ok) {
        throw new Error('Bad row: ' + row);
    } 
}).then(function () {
    console.log('done processing'); 
}).catch(function (error) {
    console.log('Error:', error.message);
});
```

__Example:__ As above, but using the `rowFinished` and `final` callbacks rather than the Promise returned from `eachAsync`.

```javascript
cursor.eachAsync(
    function (row, rowFinished) {
        var ok = processRowData(row);
        if (ok) {
            rowFinished();
        } else {
            rowFinished('Bad row: ' + row);
        }
    },
    function (error) {
        if (error) {
            console.log('Error:', error.message);
        } else {
            console.log('done processing');
        }
    }
);
```

__Note:__ You need to manually close the cursor if you prematurely stop the iteration.
