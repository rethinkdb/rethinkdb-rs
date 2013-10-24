---
layout: api-command 
language: Ruby
permalink: api/ruby/reduce/
command: reduce 
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

```rb
r.table('marvel').order_by(:strength)[5..10].run(conn)
```


