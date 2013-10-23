---
layout: api-command 
language: Python
permalink: api/python/avg/
command: avg
github_doc: https://github.com/rethinkdb/docs/blob/master/2-query-language/api/python/aggregation/avg.md
related_commands:
    group_by: group_by/
    count: count-aggregator/
    sum: sum/
---

{% apibody %}
r.avg(attr)
{% endapibody %}

Compute the average value of the given attribute for the group.

__Example:__ What's the average agility of heroes at each strength level?

```py
r.table('marvel').group_by('strength', r.avg('agility')).run(conn)
```
