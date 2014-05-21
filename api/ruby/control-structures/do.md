---
layout: api-command
language: Ruby
permalink: api/ruby/do/
command: do
---

# Command syntax #

{% apibody %}
any.do([args*, ]expr) &rarr; any
{% endapibody %}

# Description #

Evaluate the expression in the context of one or more value bindings. The values to be bound may be passed in as arguments to `do`, or chained from previous commands. The last argument is always the expression to evaluate. The type of the result is the type of the value returned from the expression.

__Example:__ Compute a golfer's net score for a game using `do` to bind to the retrieved document.


```rb
r.table('players').get('f19b5f16-ef14-468f-bd48-e194761df255').do { |player|
    player['gross_score'] - player['course_handicap']
}.run(conn)
```
