---
layout: api-command 
language: JavaScript
permalink: api/javascript/hasNext/
command: hasNext 
io:
    -   - cursor
        - undefined
related_commands:
    next: next/
    each: each/
    toArray: toArray/
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


