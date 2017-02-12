---
layout: api-command 
language: Java
permalink: api/java/sync/
command: sync
related_commands:
    noreplyWait: noreply_wait/
---

# Command syntax #

{% apibody %}
table.sync() &rarr; object
{% endapibody %}

# Description #

Ensure that writes on a given table are written to permanent storage. Queries that specify soft durability do not wait for writes to be committed to disk; a call to `sync` on a table will not return until all previous writes to the table are completed, guaranteeing the data's persistence.

If successful, the operation returns an object: `{"synced": 1}`.

__Example:__ After having updated multiple heroes with soft durability, we now want to wait
until these changes are persisted.

```java
r.table("marvel").sync().run(conn);
```


