---
layout: api-command
language: Python
permalink: api/python/change_at/
command: change_at
related_commands:
    insert_at: insert_at/
    splice_at: splice_at/
    delete_at: delete_at
---

# Command syntax #

{% apibody %}
array.change_at(offset, value) &rarr; array
{% endapibody %}

# Description #

Change a value in an array at a given index. Returns the modified array.

__Example:__ Bruce Banner hulks out.

```py
r.expr(["Iron Man", "Bruce", "Spider-Man"]).change_at(1, "Hulk").run(conn)
```
