---
layout: api-command 
language: JavaScript
permalink: api/javascript/grouped_map_reduce/
command: groupedMapReduce 
github_doc: https://github.com/rethinkdb/docs/blob/master/2-query-language/api/javascript/aggregation/groupedMapReduce.md
io:
    -   - sequence
        - value
related_commands:
    map: map/
    reduce: reduce/
    groupBy: group_by/
---

# Command syntax #

{% apibody %}
sequence.groupedMapReduce(grouping, mapping, reduction, base)
    &rarr; value
{% endapibody %}

# Description #

Partition the sequence into groups based on the `grouping` function. The elements of each
group are then mapped using the `mapping` function and reduced using the `reduction`
function.

`grouped_map_reduce` is a generalized form of group by.

__Example:__ It's only fair that heroes be compared against their weight class.

```js
r.table('marvel').groupedMapReduce(
    function(hero) { return hero('weightClass')},  // grouping
    function(hero) { return hero.pluck('name', 'strength')},  // mapping
    function(acc, hero) {  // reduction
        return r.branch(acc('strength').lt(hero('strength')), hero, acc)
    },
    {name:'none', strength:0} // reduction base
).run(conn, callback)
```


