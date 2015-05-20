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

The result is an array containing one object for each table index:

```rb
{
    :index => <index_name>,
    :ready => true,
    :function => <binary>,
    :multi => <bool>,
    :geo => <bool>,
    :outdated => <bool>
}
```

See the [index_status](/api/ruby/index_status) documentation for a description of the field values.


__Example:__ Wait for all indexes on the table `test` to be ready:

```rb
r.table('test').index_wait.run(conn)
```

__Example:__ Wait for the index `timestamp` to be ready:

```rb
r.table('test').index_wait('timestamp').run(conn)
```
