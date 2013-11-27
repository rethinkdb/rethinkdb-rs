---
layout: api-command
language: Python
permalink: api/python/branch/
command: branch
---

# Command syntax #

{% apibody %}
r.branch(test, true_branch, false_branch) &rarr; any
{% endapibody %}

# Description #

If the `test` expression returns `False` or `None`, the `false_branch` will be executed.
In the other cases, the `true_branch` is the one that will be evaluated.
   
The `branch` command is effectively an `if` renamed due to language constraints.

__Example:__ Return heroes and superheroes.

```py
r.table('marvel').map(
    r.branch(
        r.row['victories'] > 100,
        r.row['name'] + ' is a superhero',
        r.row['name'] + ' is a hero'
    )
).run(conn)
```

If the documents in the table `marvel` are:

```py
[{
    "name": "Iron man",
    "victories": 214
},
{
    "name": "Jubilee",
    "victories": 9
}]
```

The results will be:

```py
[
    "Iron man is a superhero",
    "Jubilee is a hero"
]
```
