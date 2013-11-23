---
layout: api-command
language: Ruby
permalink: api/ruby/type_of/
command: type_of
---

# Command syntax #

{% apibody %}
any.type_of() &rarr; string
{% endapibody %}

# Description #

Gets the type of a value.

__Example:__ Get the type of a string.

```rb
r.expr("foo").type_of().run(conn)
```


