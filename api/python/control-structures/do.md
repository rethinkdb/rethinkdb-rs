---
layout: api-command
language: Python
permalink: api/python/do/
command: do
---

# Command syntax #

{% apibody %}
any.do([args*, ]expr) &rarr; any
{% endapibody %}

# Description #

Evaluate the expression in the context of one or more value bindings. The values to be bound may be passed in as arguments to `do`, or chained from previous commands. The last argument is always the expression to evaluate. The type of the result is the type of the value returned from the expression.

__Example:__ Compute a golfer's net score for a game using `do` to bind to the retrieved document.

```py
r.table('players').get('86be93eb-a112-48f5-a829-15b2cb49de1d').do(
    lambda player: player['gross_score'] - player['course_handicap']
).run(conn)
```


