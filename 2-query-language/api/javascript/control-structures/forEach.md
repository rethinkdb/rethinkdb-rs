---
layout: api-command 
language: JavaScript
permalink: api/javascript/for_each/
command: forEach 
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/javascript/control-structures/forEach.md
io:
    -   - sequence
        - object
related_commands:
    map: map/
---

{% apibody %}
sequence.forEach(write_query) &rarr; object
{% endapibody %}

Loop over a sequence, evaluating the given write query for each element.

__Example:__ Now that our heroes have defeated their villains, we can safely remove them from the villain table.

```js
r.table('marvel').forEach(function(hero) {
    return r.table('villains').get(hero('villainDefeated')).delete()
}).run(conn, callback)
```
