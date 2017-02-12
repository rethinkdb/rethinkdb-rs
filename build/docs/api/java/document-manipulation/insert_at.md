---
layout: api-command
language: Java
permalink: api/java/insert_at/
command: insertAt
related_commands:
    spliceAt: splice_at/
    deleteAt: delete_at/
    changeAt: change_at/
---

# Command syntax #

{% apibody %}
array.insertAt(offset, value) &rarr; array
{% endapibody %}

# Description #

Insert a value in to an array at a given index. Returns the modified array.

__Example:__ Hulk decides to join the avengers.

```java
r.expr(r.array("Iron Man", "Spider-Man")).insertAt(1, "Hulk").run(conn);
```


