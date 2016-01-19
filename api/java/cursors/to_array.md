---
layout: api-command
language: Java
permalink: api/java/to_array/
command: toList
related_commands:
    next: next/
    for: each/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.toList()
{% endapibody %}

# Description #

Retrieve all results from a cursor as a list.

RethinkDB cursors can be iterated through via the Java [Iterable][i1] and [Iterator][i2] interfaces; to coerce a cursor into a list, use `toList`.

[i1]: https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html
[i2]: https://docs.oracle.com/javase/8/docs/api/java/util/Iterator.html
[for]: /api/java/each
[toList]: /api/java/to_array

__Example:__ For small result sets it may be more convenient to process them at once as a list.

```java
Cursor cursor = r.table("users").run(conn);
List users = cursor.toList();
processResults(users);
```

<!-- stop -->

The equivalent query with a `for` loop would be:

```java
Cursor cursor = r.table("users").run(conn);
for (Object doc : cursor) {
    processResults(doc);
}
```

__Note:__ Because a feed is a cursor that never terminates, using `list` with a feed will never return. Use [for](../each/) or [next](../next/) instead. See the [changes](/api/java/changes) command for more information on feeds.
