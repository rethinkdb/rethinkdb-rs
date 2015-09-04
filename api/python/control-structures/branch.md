---
layout: api-command
language: Python
permalink: api/python/branch/
command: branch
related_commands:
    do: do/
---

# Command syntax #

{% apibody %}
r.branch(test, true_action[, test2, else_action, ...], false_action) &rarr; any
{% endapibody %}

# Description #

Perform a branching conditional equivalent to `if-then-else`.

The `branch` command takes 2n+1 arguments: pairs of conditional expressions and commands to be executed if the conditionals return any value but `False` or `None` (i.e., "truthy" values), with a final "else" command to be evaluated if all of the conditionals are `False` or `None`.

```
r.branch(test1, val1, test2, val2, elseval)
```

is the equivalent of the Python statement

```py
if test1:
    return val1
elif test2:
    return val2
else:
    return elseval
```

__Example:__ Test the value of x.

```py
x = 10
r.branch((x > 5), 'big', 'small').run(conn)

> "big"
```

__Example:__ Categorize heroes by victory counts.

```py
r.table('marvel').map(
    r.branch(
        r.row['victories'] > 100,
        r.row['name'] + ' is a superhero',
        r.row['victories'] > 10,
        r.row['name'] + ' is a hero',
        r.row['name'] + ' is very nice'
    )
).run(conn)
```

If the documents in the table `marvel` are:

```py
[
    { "name": "Iron Man", "victories": 214 },
    { "name": "Jubilee", "victories": 49 },
    { "name": "Slava", "victories": 5 }
]
```

The results will be:

```py
[
    "Iron Man is a superhero",
    "Jubilee is a hero",
    "Slava is very nice"
]
```
