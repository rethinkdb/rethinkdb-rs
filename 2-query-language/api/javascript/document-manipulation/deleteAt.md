---
layout: api-command 
language: JavaScript
permalink: api/javascript/delete_at/
command: deleteAt
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/document-manipulation/deleteAt.md
io:
    -   - array
        - array
related_commands:
    insertAt: insert_at/
    spliceAt: splice_at/
    changeAt: change_at/
---

{% apibody %}
array.deleteAt(index [,endIndex]) &rarr; array
{% endapibody %}

Remove an element from an array at a given index. Returns the modified array.

__Example:__ Hulk decides to leave the avengers.

```js
r.expr(["Iron Man", "Hulk", "Spider-Man"]).deleteAt(1).run(conn, callback)
```


__Example:__ Hulk and Thor decide to leave the avengers.

```js
r.expr(["Iron Man", "Hulk", "Thor", "Spider-Man"]).deleteAt(1,3).run(conn, callback)
```

