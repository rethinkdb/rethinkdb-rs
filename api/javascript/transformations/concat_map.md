---
layout: api-command
language: JavaScript
permalink: api/javascript/concat_map/
command: concatMap
io:
    -   - sequence
        - stream
    -   - array
        - array
related_commands:
    map: map/
---

# Command syntax #

{% apibody %}
sequence.concatMap(mappingFunction) &rarr; stream
array.concatMap(mappingFunction) &rarr; array
{% endapibody %}

# Description #

Flattens a sequence of arrays returned by the mappingFunction into a single sequence.

__Example:__ Construct a sequence of all monsters defeated by Marvel heroes. Here the field
'defeatedMonsters' is a list that is concatenated to the sequence.

```js
r.table('marvel').concatMap(function(hero) {
    return hero('defeatedMonsters')
}).run(conn, callback)
```


