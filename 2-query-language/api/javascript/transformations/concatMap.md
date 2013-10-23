---
layout: api-command 
language: JavaScript
permalink: api/javascript/concat_map/
command: concatMap
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/transformations/concatMap.md
io:
    -   - sequence
        - stream
    -   - array
        - array
related_commands:
    map: map/
---

{% apibody %}
sequence.concatMap(mappingFunction) → stream
array.concatMap(mappingFunction) → array
{% endapibody %}

Flattens a sequence of arrays returned by the mappingFunction into a single sequence.

__Example:__ Construct a sequence of all monsters defeated by Marvel heroes. Here the field
'defeatedMonsters' is a list that is concatenated to the sequence.

```js
r.table('marvel').concatMap(function(hero) {
    return hero('defeatedMonsters')
}).run(conn, callback)
```


