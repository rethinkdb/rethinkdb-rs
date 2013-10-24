---
layout: api-command 
language: JavaScript
permalink: api/javascript/change_at/
command: changeAt 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/document-manipulation/changeAt.md
io:
    -   - array
        - array
related_commands:
    insertAt: insert_at/
    spliceAt: splice_at/
    deleteAt: delete_at/
---

# Command syntax #

{% apibody %}
array.changeAt(index, value) &rarr; array
{% endapibody %}

# Description #

Change a value in an array at a given index. Returns the modified array.

__Example:__ Bruce Banner hulks out.

```js
r.expr(["Iron Man", "Bruce", "Spider-Man"]).changeAt(1, "Hulk").run(conn, callback)
```
