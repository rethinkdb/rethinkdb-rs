---
layout: api-command 
language: Python
permalink: api/python/splice_at/
command: splice_at 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/document-manipulation/splice_at.md
related_commands:
    insert_at: insert_at/
    delete_at: delete_at/
    change_at: change_at/
---

{% apibody %}
array.splice_at(index, array) → array
{% endapibody %}

Insert several values in to an array at a given index. Returns the modified array.

__Example:__ Hulk and Thor decide to join the avengers.

```py
r.expr(["Iron Man", "Spider-Man"]).splice_at(1, ["Hulk", "Thor"]).run(conn)
```
