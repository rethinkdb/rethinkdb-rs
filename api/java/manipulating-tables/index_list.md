---
layout: api-command
language: Java
permalink: api/java/index_list/
command: indexList
related_commands:
    indexCreate: index_create/
    indexDrop: index_drop/

---


# Command syntax #

{% apibody %}
table.indexList() &rarr; array
{% endapibody %}

# Description #

List all the secondary indexes of this table.

__Example:__ List the available secondary indexes for this table.

```java
r.table('marvel').indexList().run(conn);
```

