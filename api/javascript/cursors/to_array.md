---
layout: api-command
language: JavaScript
permalink: api/javascript/to_array/
command: toArray
py: false
rb: false
io:
    -   - cursor
        - undefined
related_commands:
    next: next/
    each: each/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.toArray(callback)
array.toArray(callback)
cursor.toArray() &rarr; promise
array.toArray() &rarr; promise
{% endapibody %}

# Description #

Retrieve all results and pass them as an array to the given callback.

_Note:_ Because a feed is a cursor that will never ends, calling `toArray` on a feed
will throw an error.

__Example:__ For small result sets it may be more convenient to process them at once as
an array.

```js
cursor.toArray(function(err, results) {
    if (err) throw err;
    processResults(results);
});
```

The equivalent query with the `each` command would be:

```js
var results = []
cursor.each(function(err, row) {
    if (err) throw err;
    results.push(row);
}, function(err, results) {
    if (err) throw err;
    processResults(results);
});
```

An equivalent query using promises.

```js
cursor.toArray().then(function(results) {
    processResults(results);
}).error(console.log);
```

