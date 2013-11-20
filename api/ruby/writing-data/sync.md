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

Makes sure that writes on the given table are written to permanent storage.
If you perform a write with `durability` set to `'soft'`,
you receive no information on when the write actually gets written to permanent storage.
A subsequent call to `sync` does not return before all previous writes to the table
have been persisted.

If successful, the operation returns an object: `{"synced": 1}`.

__Example:__ After having updated multiple heroes with soft durability, we now want to wait
until these changes have been persisted.

```rb
r.table('marvel').sync().run(conn)
```


