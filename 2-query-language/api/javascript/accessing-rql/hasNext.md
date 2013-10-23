---
layout: api-command 
language: JavaScript
permalink: api/javascript/hasNext/
command: hasNext 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/accessing-rql/hasNext.md
io:
    -   - cursor
        - undefined
related_commands:
    next: next/
    each: each/
    toArray: toArray/
    close (cursor): close-cursor/
---

{% apibody %}
cursor.hasNext() &rarr; bool
{% endapibody %}

Check if there are more elements in the cursor.

__Example:__ Are there more elements in the cursor?

```js
var hasMore = cur.hasNext();
```


