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
    hasNext: has_next/
    each: each/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.toArray(callback)
{% endapibody %}

# Description #

Retrieve all results and pass them as an array to the given callback.

__Example:__ For small result sets it may be more convenient to process them at once as
an array.

```js
cur.toArray(function(err, results) {
    for(var i in results) {
        processRow(results[i]);
    }
});
```
