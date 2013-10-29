---
layout: api-command 
language: JavaScript
permalink: api/javascript/avg/
command: avg
io:
    -   - r
        - null
related_commands:
    groupBy: group_by/
    sum: sum/
    count: count/
---

# Command syntax #

{% apibody %}
r.avg(attr)
{% endapibody %}

# Description #

Compute the average value of the given attribute for the group.

__Example:__ What's the average agility of heroes at each strength level?

```js
r.table('marvel').groupBy('strength', r.avg('agility')).run(conn, callback)
```


