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

If the `test` expression returns `false` or `null`, the `false_branch` will be executed.
In the other cases, the `true_branch` is the one that will be evaluated.

The `branch` command is effectively an `if` renamed due to language constraints.
The type of the result is determined by the type of the branch that gets executed.

__Example:__ Return heroes and superheroes.

```js
r.table('marvel').map(
    r.branch(
        r.row('victories').gt(100),
        r.row('name').add(' is a superhero'),
        r.row('name').add(' is a hero')
    )
).run(conn, callback)
```

If the documents in the table `marvel` are:

```js
[{
    name: "Iron man",
    victories: 214
},
{
    name: "Jubilee",
    victories: 9
}]
```

The results will be:

```js
[
    "Iron man is a superhero",
    "Jubilee is a hero"
]
```

