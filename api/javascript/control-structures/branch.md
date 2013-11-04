---
layout: api-command 
language: JavaScript
permalink: api/javascript/branch/
command: branch 
io:
    -   - r
        - any
related_commands:
    do: do/
---

# Command syntax #

{% apibody %}
r.branch(test, true_branch, false_branch) &rarr; any
{% endapibody %}

# Description #

Evaluate one of two control paths based on the value of an expression. branch is effectively an if renamed due to language constraints.

The type of the result is determined by the type of the branch that gets executed.

__Example:__ Return the manlier of two heroes:

```
r.table('marvel').map(r.branch(r.row('victories').gt(100),
    r.row('name').add(' is a superhero'),
    r.row('name').add(' is a hero'))
).run(conn, callback)
```


