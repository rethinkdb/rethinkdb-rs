---
layout: api-command
language: JavaScript
permalink: api/javascript/sum/
command: sum
io:
    -   - r
        - null
related_commands:
    groupBy: group_by/
    count: count/
    avg: avg/
---

# Command syntax #

{% apibody %}
r.sum(attr)
{% endapibody %}

# Description #

Compute the sum of the given field in the group.

__Example:__ How many enemies have been vanquished by heroes at each strength level?

```js
r.table('marvel').groupBy('strength', r.sum('enemiesVanquished')).run(conn, callback)
```

