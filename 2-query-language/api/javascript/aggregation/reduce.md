---
layout: api-command 
language: JavaScript
permalink: api/javascript/reduce/
command: reduce 
github_doc: https://github.com/rethinkdb/docs/blob/master/2-query-language/api/javascript/aggregation/reduce.md
io:
    -   - sequence
        - value
related_commands:
    map: map/
    groupedMapReduce: grouped_map_reduce/
---

{% apibody %}
sequence.reduce(reductionFunction[, base]) → value
{% endapibody %}

Produce a single value from a sequence through repeated application of a reduction
function.

The reduce function gets invoked repeatedly not only for the input values but also for
results of previous reduce invocations. The type and format of the object that is passed
in to reduce must be the same with the one returned from reduce.

__Example:__ How many enemies have our heroes defeated?

```js
r.table('marvel').map(r.row('monstersKilled')).reduce(function(acc, val) {
    return acc.add(val)
}, 0).run(conn, callback)
```
