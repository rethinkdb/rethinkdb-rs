---
layout: api-command
language: Java
permalink: api/java/next/
command: next
related_commands:
    for: each/
    toList: to_array/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.next([wait])
{% endapibody %}

# Description #

Get the next element in the cursor.

The optional argument specifies whether to wait for the next available element and how long to wait:

* `true`: Wait indefinitely (the default).
* `false`: Do not wait at all. If data is immediately available, it will be returned; if it is not available, a `ReqlTimeoutError` will be raised.
* number: Wait up to the specified number of seconds for data to be available before raising `ReqlTimeoutError`.

The behavior of `next` will be identical with `false`, `null` or the number `0`.

Calling `next` the first time on a cursor provides the first element of the cursor. If the data set is exhausted (e.g., you have retrieved all the documents in a table), a `NoSuchElementException` error will be raised when `next` is called.

__Example:__ Retrieve the next element.

```java
Cursor cursor = r.table("superheroes").run(conn);
Object doc = cursor.next();
```

__Example:__ Retrieve the next element on a [changefeed](/docs/changefeeds/java), waiting up to five seconds.

```java
Cursor cursor = r.table("superheroes").changes().run(conn);
Object doc = cursor.next(5);
```

__Note:__ RethinkDB cursors can be iterated through via the Java [Iterable][i1] and [Iterator][i2] interfaces. The canonical way to retrieve all the results is to use a [for][] loop or [toList][].

[i1]: https://docs.oracle.com/javase/8/docs/api/java/lang/Iterable.html
[i2]: https://docs.oracle.com/javase/8/docs/api/java/util/Iterator.html
[for]: /api/java/each
[toList]: /api/java/to_array
