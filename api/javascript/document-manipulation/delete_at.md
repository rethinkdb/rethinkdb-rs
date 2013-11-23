---
layout: api-command
language: JavaScript
permalink: api/javascript/delete_at/
command: deleteAt
io:
    -   - array
        - array
related_commands:
    insertAt: insert_at/
    spliceAt: splice_at/
    changeAt: change_at/
---

# Command syntax #

{% apibody %}
array.deleteAt(index [,endIndex]) &rarr; array
{% endapibody %}

# Description #

Remove an element from an array at a given index. Returns the modified array.

__Example:__ Hulk decides to leave the avengers.

```js
r.expr(["Iron Man", "Hulk", "Spider-Man"]).deleteAt(1).run(conn, callback)
```


__Example:__ Hulk and Thor decide to leave the avengers.

```js
r.expr(["Iron Man", "Hulk", "Thor", "Spider-Man"]).deleteAt(1,3).run(conn, callback)
```

