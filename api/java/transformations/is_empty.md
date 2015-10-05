---
layout: api-command
language: Java
permalink: api/java/is_empty/
command: isEmpty
related_commands:
    offsetsOf: offsets_of/
io:
    -   - sequence
        - bool
---

# Command syntax #

{% apibody %}
sequence.isEmpty() &rarr; bool
{% endapibody %}

# Description #

Test if a sequence is empty.

__Example:__ Are there any documents in the marvel table?

```java
r.table('marvel').isEmpty().run(conn)
```
