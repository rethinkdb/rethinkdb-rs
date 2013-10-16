---
layout: api-command 
language: Python
permalink: api/python/delete_at/
command: delete_at
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/document-manipulation/delete_at.md
---

{% apibody %}
array.delete_at(index [,endIndex]) &rarr; array
{% endapibody %}

Remove an element from an array at a given index. Returns the modified array.

__Example:__ Hulk decides to leave the avengers.

```py
r.expr(["Iron Man", "Hulk", "Spider-Man"]).delete_at(1).run(conn)
```

__Example:__ Hulk and Thor decide to leave the avengers.

```py
r.expr(["Iron Man", "Hulk", "Thor", "Spider-Man"]).delete_at(1,3).run(conn)
```

