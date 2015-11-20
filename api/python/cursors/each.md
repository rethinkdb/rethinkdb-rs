---
layout: api-command
language: Python
permalink: api/python/each/
command: for
related_commands:
    next: next/
    list: to_array/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
for items in cursor:
for items in array:
for items in feed:
{% endapibody %}

# Description #

Lazily iterate over a result set one element at a time.

RethinkDB sequences can be iterated through via the Python [Iterable][it] interface; use standard Python commands like `for` loops to access each item in the sequence.

[it]: https://docs.python.org/3.4/library/stdtypes.html#iterator-types


__Example:__ Let's process all the elements!

```py
cursor = r.table('users').run(conn)
for doc in cursor:
    process_row(doc)
```

__Example:__ Stop the iteration prematurely and close the connection manually.

```py
cursor = r.table('users').run(conn)
for doc in cursor:
    ok = process_row(doc)
    if ok is False:
        cursor.close()
        break
```
