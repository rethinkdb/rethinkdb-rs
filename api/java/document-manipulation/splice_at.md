---
layout: api-command
language: JavaScript
permalink: api/javascript/splice_at/
command: spliceAt
io:
    -   - array
        - array
related_commands:
    insertAt: insert_at/
    deleteAt: delete_at/
    changeAt: change_at/
---

# Command syntax #

{% apibody %}
array.spliceAt(index, array) &rarr; array
{% endapibody %}

# Description #

Insert several values in to an array at a given index. Returns the modified array.

__Example:__ Hulk and Thor decide to join the avengers.

```js
r.expr(["Iron Man", "Spider-Man"]).spliceAt(1, ["Hulk", "Thor"]).run(conn)
```

