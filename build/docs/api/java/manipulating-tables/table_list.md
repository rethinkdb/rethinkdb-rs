---
layout: api-command
language: Java
permalink: api/java/table_list/
command: tableList
related_commands:
    tableCreate: table_create/
    tableDrop: table_drop/
---

# Command syntax #

{% apibody %}
db.tableList() &rarr; array
{% endapibody %}

# Description #

List all table names in a database. The result is a list of strings.

__Example:__ List all tables of the 'test' database.

```java
r.db("test").tableList().run(conn);
```

