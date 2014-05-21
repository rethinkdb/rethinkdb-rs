---
layout: api-command
language: JavaScript
permalink: api/javascript/do/
command: do
io:
    -   - any
        - any
---

# Command syntax #

{% apibody %}
any.do([args*, ]expr) &rarr; any
{% endapibody %}

# Description #

Evaluate the expression in the context of one or more value bindings. The values to be bound may be passed in as arguments to `do`, or chained from previous commands. The last argument is always the expression to evaluate. The type of the result is the type of the value returned from the expression.

__Example:__ Compute a golfer's net score for a game using `do` to bind to the retrieved document.

```js
r.table('players').get('f19b5f16-ef14-468f-bd48-e194761df255').do(
  function(player) {
    return player('gross_score').sub(player('course_handicap'));
  }
).run(conn, callback);
```