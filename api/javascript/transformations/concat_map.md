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
stream.concatMap(mappingFunction) &rarr; stream
array.concatMap(mappingFunction) &rarr; array
{% endapibody %}

# Description #

Concatenate one or more sequences into a single sequence using a mapping function. This works in a similar fashion as `map`, but each element of the sequence the mapping function is applied to is itself a sequence rather than a single value. The return value will be the same type as the input value.

__Example:__ Construct a sequence of all monsters defeated by Marvel heroes. The field "defeatedMonsters" is an array of one or more monster names.

```js
r.table('marvel').concatMap(function(hero) {
    return hero('defeatedMonsters')
}).run(conn, callback)
```


