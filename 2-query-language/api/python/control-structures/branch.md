---
layout: api-command 
language: Python
permalink: api/python/branch/
command: branch 
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/python/control-structures/branch.md
---

{% apibody %}
r.branch(test, true_branch, false_branch) → any
{% endapibody %}

Evaluate one of two control paths based on the value of an expression. branch is effectively an if renamed due to language constraints.

The type of the result is determined by the type of the branch that gets executed.

__Example:__ Return the manlier of two heroes:

```py
r.table('marvel').map(r.branch(r.row['victories'] > 100,
    r.row['name'] + ' is a superhero',
    r.row['name'] + ' is a hero')
).run(conn)
```


