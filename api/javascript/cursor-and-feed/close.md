---
layout: api-command
language: JavaScript
permalink: api/javascript/close-cursor/
command: close
io:
    -   - cursor
        - undefined
related_commands:
    next: next/
    toArray: to_array/
    each: each/
---

# Command syntax #

{% apibody %}
cursor.close()
feed.close()
{% endapibody %}

# Description #


Close a cursor or a feed. Closing a cursor cancels the corresponding query and frees the memory
associated with the open request.

__Example:__ Close a cursor.

```js
cursor.close()
```
