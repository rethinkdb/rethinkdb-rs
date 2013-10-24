---
layout: api-command 
language: JavaScript
permalink: api/javascript/map/
command: map
io:
    -   - sequence
        - stream
    -   - array
        - array
related_commands:
    concatMap: concat_map/
    reduce: reduce/
---

# Command syntax #

{% apibody %}
sequence.map(mappingFunction) &rarr; stream
array.map(mappingFunction) &rarr; array
{% endapibody %}

# Description #

Transform each element of the sequence by applying the given mapping function.

__Example:__ Construct a sequence of hero power ratings.

```js
r.table('marvel').map(function(hero) {
    return hero('combatPower').add(hero('compassionPower').mul(2))
}).run(conn, callback)
```



