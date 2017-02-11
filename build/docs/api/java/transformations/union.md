---
layout: api-command
language: Java
permalink: api/java/union/
command: union
io:
    -   - sequence
        - array
---

# Command syntax #

{% apibody %}
stream.union(sequence[, sequence, ...]) &rarr; stream
array.union(sequence[, sequence, ...]) &rarr; array
r.union(stream, sequence[, sequence, ...]) &rarr; stream
r.union(array, sequence[, sequence, ...]) &rarr; array
{% endapibody %}

# Description #

Merge two or more sequences.

The `interleave` [optArg](/api/java/optarg) controls how the sequences will be merged:

* `true`: results will be mixed together; this is the fastest setting, but ordering of elements is not guaranteed. (This is the default.)
* `false`: input sequences will be appended to one another, left to right.
* `"field_name"`: a string will be taken as the name of a field to perform a merge-sort on. The input sequences must be ordered _before_ being passed to `union`.
* function: the `interleave` optArg can take a function whose argument is the current row, and whose return value is a string to take as a field name, as with the `"field_name"` setting described above.

__Example:__ Construct a stream of all heroes.

```java
r.table("marvel").union(r.table("dc")).run(conn);
```

__Example:__ Combine four arrays into one.

```java
r.expr(r.array(1, 2)).union(r.array(3, 4), r.array(5, 6), r.array(7, 8, 9)).run(conn);

// Result:
[1, 2, 3, 4, 5, 6, 7, 8, 9]
```

__Example:__ Create a [changefeed][cf] from the first example.

```java
r.table("marvel").union(r.table("dc")).changes().run(conn);
```

Now, when any heroes are added, modified or deleted from either table, a change notification will be sent out.

[cf]: /docs/changefeeds/java

__Example:__ Merge-sort the tables of heroes, ordered by name.

```java
r.table("marvel").orderBy("name")
 .union(r.table("dc").orderBy("name")).optArg("interleave", "name")
 .run(conn);
```
