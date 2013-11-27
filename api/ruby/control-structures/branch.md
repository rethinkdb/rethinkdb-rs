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

If the `test` expression returns `false` or `nil`, the `false_branch` will be executed.
In the other cases, the `true_branch` is the one that will be evaluated.
   
The `branch` command is effectively an `if` renamed due to language constraints.
The type of the result is determined by the type of the branch that gets executed.

__Example:__ Return heroes and superheroes.


```rb
r.table('marvel').map{ |hero|
    r.branch(
        hero['victories'] > 100,
        hero['name'].add(' is a superhero'),
        hero['name'].add(' is a hero')
    )
}.run(conn)
```

If the documents in the table `marvel` are:

```rb
[{
    :name => "Iron man",
    :victories => 214
},
{
    :name => "Jubilee",
    :victories => 9
}]
```

The results will be:

```rb
[
    "Iron man is a superhero",
    "Jubilee is a hero"
]
```
