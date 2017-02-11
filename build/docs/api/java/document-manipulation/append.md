---
layout: api-command
language: Java
permalink: api/java/append/
command: append
related_commands:
    prepend: prepend/
    insertAt: insert_at/
    deleteAt: delete_at/
    changeAt: change_at/
---
# Command syntax #

{% apibody %}
array.append(value) &rarr; array
{% endapibody %}

# Description #

Append a value to an array.

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```java
r.table("marvel").get("IronMan").g("equipment").append("newBoots").run(conn);
```


