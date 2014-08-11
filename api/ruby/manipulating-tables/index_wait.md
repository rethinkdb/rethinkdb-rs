---
layout: api-command
language: Ruby
permalink: api/ruby/index_wait/
command: index_wait
related_commands:
    index_status: index_status/
---

# Command syntax #

{% apibody %}
table.index_wait([, index...]) &rarr; array
{% endapibody %}

# Description #


Wait for the specified indexes on this table to be ready, or for all
indexes on this table to be ready if no indexes are specified.

The result is an array where for each index, there will be an object like:

```rb
{
    :index => <index_name>,
    :ready => true,
    :multi => <bool>,
    :outdated => <bool>
}
```

The `multi` field will be `true` or `false` depending on whether this index was created as a multi index (see [index_create](/api/ruby/index_create/) for details). The `outdated` field will be true if the index is outdated in the current version of RethinkDB and needs to be rebuilt.


__Example:__ Wait for all indexes on the table `test` to be ready:

```rb
r.table('test').index_wait.run(conn)
```

__Example:__ Wait for the index `timestamp` to be ready:

```rb
r.table('test').index_wait('timestamp').run(conn)
```
