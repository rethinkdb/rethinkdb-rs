---
layout: api-command 
language: JavaScript
permalink: api/javascript/each/
command: each
py: false
rb: false
io:
    -   - cursor
        - undefined
related_commands:
    next: next/
    hasNext: has_next/
    toArray: to_array/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.each(callback[, onFinishedCallback])
array.each(callback[, onFinishedCallback])
{% endapibody %}

# Description #

Lazily iterate over the result set one element at a time. The second callback is optional
and is called when the iteration stops (when there are no more rows or when the callback
returns `false`).

__Example:__ Let's process all the elements!

```js
cursor.each(function(err, row) {
    if (err) throw err;
    processRow(row);
});
```

__Example:__ If we need to know when the iteration is complete, `each` also accepts a second `onFinished` callback.

```js
cursor.each(function(err, row) {
        if (err) throw err;
        processRow(row);
    }, function() {
        doneProcessing();
    }
);
```


__Example:__ Iteration can be stopped prematurely by returning `false` from the callback.
For instance, if you want to stop the iteration as soon as `row` is negative.

```js
cursor.each(function(err, row) {
    if (err) throw err;

    if (row < 0) {
        cursor.close()
        return false;
    }
    else {
        processRow(row)
    }
});
```

__Note:__ You need to manually close the cursor if you prematurely stop the iteration.
