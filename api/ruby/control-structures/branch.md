---
layout: api-command 
language: Ruby
permalink: api/ruby/branch/
command: branch 
---

# Command syntax #

{% apibody %}
r.branch(test, true_branch, false_branch) &rarr; any
{% endapibody %}

# Description #

Evaluate one of two control paths based on the value of an expression. branch is effectively an if renamed due to language constraints.

The type of the result is determined by the type of the branch that gets executed.

__Example:__ Return the manlier of two heroes:

```rb
r.table('marvel').map { |row|  r.branch(row[:victories] > 100,
    row[:name] + ' is a superhero',
    row[:name] + ' is a hero')
}.run(conn)
```
