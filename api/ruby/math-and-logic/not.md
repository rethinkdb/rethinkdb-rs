---
layout: api-command 
language: Ruby
permalink: api/ruby/not/
command: not
related_commands:
    'eq': eq/
    'ne': ne/
---

# Command syntax #

{% apibody %}
bool.not() &rarr; bool
{% endapibody %}

# Description #
Compute the logical inverse (not).

__Example:__ Not true is false.

```rb
r(true).not.run(conn)
```
