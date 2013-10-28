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
    hasNext: hasNext/
    each: each/
    toArray: toArray/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.next(callback)
{% endapibody %}

# Description #

Get the next element in the cursor.

__Example:__ Let's grab the next element!

```js
cur.next(function(err, row) {
    return processRow(row);
});
```

