---
layout: api-command
language: Ruby
permalink: api/ruby/branch/
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

The `branch` command takes 2n+1 arguments: pairs of conditional expressions and commands to be executed if the conditionals return any value but `false` or `nil` (i.e., "truthy" values), with a final "else" command to be evaluated if all of the conditionals are `false` or `nil`.

```
r.branch(test1, val1, test2, val2, elseval)
```

is the equivalent of the Python statement

```rb
if test1
    val1
elsif test2
    val2
else
    elseval
end
```

__Example:__ Test the value of x.

```rb
x = 10
r.branch((x > 5), 'big', 'small').run(conn)

> "big"
```

__Example:__ Categorize heroes by victory counts.

```rb
r.table('marvel').map(
    r.branch(
        r.row['victories'] > 100,
        r.row['name'].add(' is a superhero',)
        r.row['victories'] > 10,
        r.row['name'].add(' is a hero',)
        r.row['name'].add(' is very nice')
    )
).run(conn)
```

If the documents in the table `marvel` are:

```rb
[
    { :name => "Iron Man", :victories => 214 },
    { :name => "Jubilee", :victories => 49 },
    { :name => "Slava", :victories => 5 }
]
```

The results will be:

```rb
[
    "Iron Man is a superhero",
    "Jubilee is a hero",
    "Slava is very nice"
]
```
