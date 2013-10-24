---
layout: api-command 
language: JavaScript
permalink: api/javascript/distinct/
command: distinct 
io:
    -   - sequence
        - value
related_commands:
    map: map/
    reduce: reduce/
    groupedMapReduce: grouped_map_reduce/
---

# Command syntax #

{% apibody %}
sequence.distinct() &rarr; array
{% endapibody %}

# Description #

Remove duplicate elements from the sequence.

__Example:__ Which unique villains have been vanquished by marvel heroes?

```js
r.table('marvel').concatMap(function(hero) {return hero('villainList')}).distinct()
    .run(conn, callback)
```
