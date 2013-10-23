---
layout: api-command 
language: Python
permalink: api/python/reduce/
command: reduce
github_doc: https://github.com/rethinkdb/docs/blob/master/2-query-language/api/python/aggregation/reduce.md
related_commands:
    map: map/
    concat_map: concat_map/
    grouped_map_reduce: grouped_map_reduce/
    group_by: group_by/
---

# Command syntax #

{% apibody %}
sequence.reduce(reduction_function[, base]) &rarr; value
{% endapibody %}

# Description #

Produce a single value from a sequence through repeated application of a reduction
function.

The reduce function gets invoked repeatedly not only for the input values but also for
results of previous reduce invocations. The type and format of the object that is passed
in to reduce must be the same with the one returned from reduce.

__Example:__ How many enemies have our heroes defeated?

```py
r.table('marvel').map(r.row['monstersKilled']).reduce(
    lambda acc, val: acc + val, 0).run(conn)
```
