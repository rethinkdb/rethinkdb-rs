---
layout: api-command 
language: JavaScript
permalink: api/javascript/map/
command: map
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/transformations/map.md
io:
    -   - sequence
        - stream
    -   - array
        - array
related_commands:
    concatMap: concat_map/
    reduce: reduce/
---

{% apibody %}
sequence.map(mappingFunction) &rarr; stream
array.map(mappingFunction) &rarr; array
{% endapibody %}

Transform each element of the sequence by applying the given mapping function.

__Example:__ Construct a sequence of hero power ratings.

```js
r.table('marvel').map(function(hero) {
    return hero('combatPower').add(hero('compassionPower').mul(2))
}).run(conn, callback)
```



