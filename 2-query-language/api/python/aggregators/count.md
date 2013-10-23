---
layout: api-command 
language: Python
permalink: api/python/count-aggregator/
command: count
github_doc: https://github.com/rethinkdb/docs/blob/master/2-query-language/api/python/aggregation/count.md
related_commands:
    group_by: group_by/
    sum: sum/
    avg: avg/
---

{% apibody %}
r.count
{% endapibody %}

Count the total size of the group.

__Example:__ Just how many heroes do we have at each strength level?

```py
r.table('marvel').group_by('strength', r.count).run(conn)
```
