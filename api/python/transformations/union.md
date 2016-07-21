---
layout: api-command
language: Python
permalink: api/python/union/
command: union
---

# Command syntax #

{% apibody %}
stream.union(sequence[, sequence, ...][, interleave=True]) &rarr; stream
array.union(sequence[, sequence, ...][, interleave=True]) &rarr; array
r.union(stream, sequence[, sequence, ...][, interleave=True]) &rarr; stream
r.union(array, sequence[, sequence, ...][, interleave=True]) &rarr; array
{% endapibody %}

# Description #

Merge two or more sequences.

The optional `interleave` argument controls how the sequences will be merged:

* `True`: results will be mixed together; this is the fastest setting, but ordering of elements is not guaranteed. (This is the default.)
* `False`: input sequences will be appended to one another, left to right.
* `"field_name"`: a string will be taken as the name of a field to perform a merge-sort on. The input sequences must be ordered _before_ being passed to `union`.
* function: the `interleave` argument can take a function whose argument is the current row, and whose return value is a string to take as a field name, as with the `"field_name"` setting described above.

__Example:__ Construct a stream of all heroes.

```py
r.table('marvel').union(r.table('dc')).run(conn)
```

__Example:__ Combine four arrays into one.

```py
r.expr([1, 2]).union([3, 4], [5, 6], [7, 8, 9]).run(conn)

[1, 2, 3, 4, 5, 6, 7, 8, 9]
```

__Example:__ Create a [changefeed][cf] from the first example.

```py
r.table('marvel').union(r.table('dc')).changes().run(conn)
```

Now, when any heroes are added, modified or deleted from either table, a change notification will be sent out.

[cf]: /docs/changefeeds/python

__Example:__ Merge-sort the tables of heroes, ordered by name.

```py
r.table('marvel').order_by('name').union(
    r.table('dc').order_by('name'), interleave='name'
).run(conn)
```
