---
layout: api-command 
language: Python
permalink: api/python/sum/
command: sum
related_commands:
    group_by: group_by/
    count: count-aggregator/
    avg: avg/
---

# Command syntax #

{% apibody %}
r.sum(attr)
{% endapibody %}

# Description #

Compute the sum of the given field in the group.

__Example:__ How many enemies have been vanquished by heroes at each strength level?

```py
r.table('marvel').group_by('strength', r.sum('enemiesVanquished')).run(conn)
```
