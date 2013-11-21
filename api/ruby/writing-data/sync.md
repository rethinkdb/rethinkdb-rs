---
layout: api-command 
language: Ruby
permalink: api/ruby/sync/
command: sync
related_commands:
    noreply_wait: noreply_wait/
---

# Command syntax #

{% apibody %}
table.sync() &rarr; object
{% endapibody %}

# Description #

`sync` ensures that writes on a given table are written to permanent storage. Queries
that specify soft durability (`{:durability => soft}`) do not give such guarantees, so
`sync` can be used to ensure the state of these queries. A call to `sync` does not return
until all previous writes to the table are persisted.

If successful, the operation returns an object: `{"synced": 1}`.

__Example:__ After having updated multiple heroes with soft durability, we now want to wait
until these changes are persisted.

```rb
r.table('marvel').sync().run(conn)
```


