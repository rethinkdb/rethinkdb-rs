---
layout: api-command 
language: JavaScript
permalink: api/javascript/close-cursor/
command: close
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/accessing-rql/close-cursor.md
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

{% apibody %}
cursor.close()
{% endapibody %}


Close a cursor. Closing a cursor cancels the corresponding query and frees the memory
associated with the open request.

__Example:__ Close a cursor.

```js
cursor.close()
```
