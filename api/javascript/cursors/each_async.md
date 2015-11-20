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
cursor.eachAsync(function) &rarr; promise
array.eachAsync(function) &rarr; promise
feed.eachAsync(function) &rarr; promise
{% endapibody %}

# Description #

Lazily iterate over a result set one element at a time in an identical fashion to [each](/api/javascript/each/), returning a Promise that will be resolved once all rows are returned.

__Example:__ Process all the elements in a stream.

```js
cursor.eachAsync(function(row) {
    // if a Promise is returned, it will be processed before the cursor
    // continues iteration.
    return asyncRowHandler(row);
}).then(function () {
    console.log("done processing"); 
});
```

__Example:__ Iteration can be stopped early by returning or throwing a promise that is rejected.

```js
cursor.eachAsync(function(row) {
    if (row.id < 10) {
        return asyncRowHandler(row);
    } else {
        return Promise.reject();
    }
}).then(function () {
    console.log("done processing"); 
});
```

__Note:__ You need to manually close the cursor if you prematurely stop the iteration.
