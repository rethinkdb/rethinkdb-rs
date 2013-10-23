---
layout: api-command 
language: JavaScript
permalink: api/javascript/avg/
command: avg
github_doc: https://github.com/rethinkdb/docs/blob/master/2-query-language/api/javascript/aggregators/avg.md
io:
    -   - r
        - null
related_commands:
    groupBy: groupBy/
    sum: sum/
    count: count/
---

{% apibody %}
r.avg(attr)
{% endapibody %}

Compute the average value of the given attribute for the group.

__Example:__ What's the average agility of heroes at each strength level?

```js
r.table('marvel').groupBy('strength', r.avg('agility')).run(conn, callback)
```


