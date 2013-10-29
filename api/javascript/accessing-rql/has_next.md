---
layout: api-command 
language: JavaScript
permalink: api/javascript/has_next/
command: hasNext 
rb: false
py: false
io:
    -   - cursor
        - undefined
related_commands:
    next: next/
    each: each/
    toArray: to_array/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.hasNext() &rarr; bool
{% endapibody %}

# Description #

Check if there are more elements in the cursor.

__Example:__ Are there more elements in the cursor?

```js
var hasMore = cur.hasNext();
```


