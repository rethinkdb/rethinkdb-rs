---
layout: api-command 
permalink: api/javascript/each/
command: each
py: false
rb: false
---

{% apibody %}
cursor.each(callback[, onFinishedCallback])
{% endapibody %}

Lazily iterate over the result set one element at a time.

__Example:__ Let's process all the elements!

```js
cur.each(function(err, row) {
    processRow(row);
});
```

__Example:__ If we need to know when iteration is complete each also accepts a second `onFinished` callback.

```js
cur.each(function(err, row) {
    return processRow(row);
}, function() {
    doneProcessing();
});
```


__Example:__ Iteration can be stopped prematurely by returning `false` from the callback.

```
cur.each(function(err, row) {
    if (processRow(row) < 0)
    return false;
});
```

