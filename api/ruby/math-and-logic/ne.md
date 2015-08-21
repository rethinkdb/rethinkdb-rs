---
layout: api-command
language: Ruby
permalink: api/ruby/ne/
command: ne
related_commands:
    '&, and': and/
    '|, or': or/
    eq: eq/
---

# Command syntax #

{% apibody %}
value.ne(value[, value, ...]) &rarr; bool
{% endapibody %}

# Description #

Test if two or more values are not equal.

__Example:__ See if a user's `role` field is not set to `administrator`. 

```rb
r.table('users').get(1)['role'].ne('administrator').run(conn)
```

__Example:__ See if three variables do not contain equal values.

```rb
r.ne(a, b, c).run(conn)
```
