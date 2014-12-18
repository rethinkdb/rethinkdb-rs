---
layout: api-command
language: Python
permalink: api/python/next/
command: next
rb: false
related_commands:
    for (cursor): each/
    list: to_array/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.next()
{% endapibody %}

# Description #

Get the next element in the cursor.

Calling `next` the first time on a cursor provides the first element of the cursor.

__Example:__ Let's grab the next element!

```py
doc = cursor.next()
```

__Note:__ RethinkDB sequences can be iterated through via the Python [Iterable][it] interface. The canonical way to retrieve all the results is to use a [for...in](../each/) loop or [list()](../to_array/).

[it]: https://docs.python.org/3.4/library/stdtypes.html#iterator-types
