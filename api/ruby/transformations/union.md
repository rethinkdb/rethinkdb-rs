---
layout: api-command
language: Ruby
permalink: api/ruby/union/
command: union
---

# Command syntax #

{% apibody %}
stream.union(sequence[, sequence, ...]) &rarr; stream
array.union(sequence[, sequence, ...]) &rarr; array
{% endapibody %}

# Description #

Merge two or more sequences. (Note that ordering is not guaranteed by `union`.)

__Example:__ Construct a stream of all heroes.

```rb
r.table('marvel').union(r.table('dc')).run(conn)
```

__Example:__ Combine four arrays into one.

```rb
r.expr([1, 2]).union([3, 4], [5, 6], [7, 8, 9]).run(conn)

[1, 2, 3, 4, 5, 6, 7, 8, 9]
```

__Example:__ Create a [changefeed][cf] from the first example.

```rb
r.table('marvel').union(r.table('dc')).changes.run(conn)
```

Now, when any heroes are added, modified or deleted from either table, a change notification will be sent out.

[cf]: /docs/changefeeds/ruby
