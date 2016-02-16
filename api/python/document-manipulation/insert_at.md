---
layout: api-command
language: Python
permalink: api/python/insert_at/
command: insert_at
related_commands:
    splice_at: splice_at/
    delete_at: delete_at/
    change_at: change_at/
---

# Command syntax #

{% apibody %}
array.insert_at(offset, value) &rarr; array
{% endapibody %}

# Description #

Insert a value in to an array at a given index. Returns the modified array.

__Example:__ Hulk decides to join the avengers.

```py
r.expr(["Iron Man", "Spider-Man"]).insert_at(1, "Hulk").run(conn)
```


