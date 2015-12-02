---
layout: api-command
language: Java
permalink: api/java/each/
command: for
related_commands:
    next: next/
    toList: to_array/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
for (doc : <Cursor>) { ... }
{% endapibody %}

# Description #

Lazily iterate over a result set one element at a time.

RethinkDB cursors can be iterated through via the Java [Iterable][i1] and [Iterator][i2] interfaces; use standard Java commands like `for` loops to access each item in the sequence.

[i1]: https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html
[i2]: https://docs.oracle.com/javase/8/docs/api/java/util/Iterator.html


__Example:__ Let's process all the elements!

```java
cursor = r.table("users").run<Cursor<Map<String, Object>>(conn);
for (Map<String, Object> doc : cursor) {
    System.out.println(doc);
}
```

__Example:__ Stop the iteration prematurely and close the connection manually.

```java
cursor = r.table("users").run<Cursor<Map<String, Object>>(conn);
for (Map<String, Object> doc : cursor) {
    ok = processRow(doc);
    if (ok == false) {
        cursor.close();
        break;
    }
}
```
