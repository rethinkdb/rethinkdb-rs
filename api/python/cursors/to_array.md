---
layout: api-command
language: Python
permalink: api/python/to_array/
command: list
related_commands:
    next: next/
    for (cursor): each/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
list(cursor)
{% endapibody %}

# Description #

Retrieve all results as a list.

RethinkDB sequences can be iterated through via the Python [Iterable][it] interface; to coerce a cursor into a list, use the Python [list()][lcc] class constructor.

[it]: https://docs.python.org/3/library/stdtypes.html#iterator-types
[lcc]: https://docs.python.org/3/library/stdtypes.html#typesseq-list


__Example:__ For small result sets it may be more convenient to process them at once as an array.

```py
cursor = r.table('users').run()
users = list(cursor)
process_results(users)
```

The equivalent query with a `for` loop would be:

```py
cursor = r.table('users').run()
for doc in cursor:
    process_results(doc)
```

__Note:__ Because a feed is a cursor that never terminates, using `list` with a feed will never return. Use [for](../each/) or [next](../next/) instead. See the [changes](/api/python/changes) command for more information on feeds.
