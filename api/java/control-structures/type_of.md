---
layout: api-command
language: Java
permalink: api/java/type_of/
command: typeOf
io:
    -   - any
        - string
---

# Command syntax #

{% apibody %}
any.typeOf() &rarr; string
{% endapibody %}

# Description #

Gets the type of a value.

__Example:__ Get the type of a string.

```java
r.expr("foo").typeOf().run(conn)
```

