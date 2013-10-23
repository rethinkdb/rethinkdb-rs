---
layout: api-command 
language: JavaScript
permalink: api/javascript/distinct/
command: distinct 
github_doc: https://github.com/rethinkdb/docs/blob/master/2-query-language/api/javascript/aggregation/distinct.md
io:
    -   - sequence
        - value
related_commands:
    map: map/
    reduce: reduce/
    groupedMapReduce: grouped_map_reduce/
---

{% apibody %}
sequence.distinct() &rarr; array
{% endapibody %}

Remove duplicate elements from the sequence.

__Example:__ Which unique villains have been vanquished by marvel heroes?

```js
r.table('marvel').concatMap(function(hero) {return hero('villainList')}).distinct()
    .run(conn, callback)
```
