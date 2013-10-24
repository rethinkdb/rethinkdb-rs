---
layout: api-command 
language: JavaScript
permalink: api/javascript/close-cursor/
command: close
rb: false
py: false
io:
    -   - cursor
        - undefined
related_commands:
    next: next/
    hasNext: hasNext/
    toArray: toArray/
    each: each/
---

# Command syntax #

{% apibody %}
cursor.close()
{% endapibody %}

# Description #


Close a cursor. Closing a cursor cancels the corresponding query and frees the memory
associated with the open request.

__Example:__ Close a cursor.

```js
cursor.close()
```
