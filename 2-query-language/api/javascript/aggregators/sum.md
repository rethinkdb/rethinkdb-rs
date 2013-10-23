---
layout: api-command 
language: JavaScript
permalink: api/javascript/sum/
command: sum 
github_doc: https://github.com/rethinkdb/docs/blob/master/2-query-language/api/javascript/aggregators/sum.md
io:
    -   - r
        - null
related_commands:
    groupBy: groupBy/
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

