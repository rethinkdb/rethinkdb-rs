---
layout: api-command
language: Python
permalink: api/python/next/
command: next
related_commands:
    for (cursor): each/
    list: to_array/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.next([wait=True])
{% endapibody %}

# Description #

Get the next element in the cursor.

The optional `wait` argument specifies whether to wait for the next available element and how long to wait:

* `True`: Wait indefinitely (the default).
* `False`: Do not wait at all. If data is immediately available, it will be returned; if it is not available, a `ReqlTimeoutError` will be raised.
* number: Wait up to the specified number of seconds for data to be available before raising `ReqlTimeoutError`.

The behavior of `next` will be identical with `False`, `None` or the number `0`.

Calling `next` the first time on a cursor provides the first element of the cursor. If the data set is exhausted (e.g., you have retrieved all the documents in a table), a `ReqlCursorEmpty` error will be raised when `next` is called.

__Example:__ Retrieve the next element.

```py
cursor = r.table('superheroes').run(conn)
doc = cursor.next()
```

__Example:__ Retrieve the next element on a [changefeed](/docs/changefeeds/python), waiting up to five seconds.

```py
cursor = r.table('superheroes').changes().run(conn)
doc = cursor.next(wait=5)
```

__Note:__ RethinkDB sequences can be iterated through via the Python [Iterable][it] interface. The canonical way to retrieve all the results is to use a [for...in](../each/) loop or [list()](../to_array/).

[it]: https://docs.python.org/3.4/library/stdtypes.html#iterator-types
