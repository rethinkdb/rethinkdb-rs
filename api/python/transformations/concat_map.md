---
layout: api-command
language: Python
permalink: api/python/concat_map/
command: concat_map
related_commands:
    map: map/
    reduce: reduce/
---

# Command syntax #

{% apibody %}
stream.concat_map(mapping_function) &rarr; stream
array.concat_map(mapping_function) &rarr; array
{% endapibody %}

# Description #

Concatenate one or more sequences into a single sequence using a mapping function. This works in a similar fashion as `map`, but each element of the sequence the mapping function is applied to is itself a sequence rather than a single value. The return value will be the same type as the input value.

__Example:__ Construct a sequence of all monsters defeated by Marvel heroes. The field "defeatedMonsters" is an array of one or more monster names.

```py
r.table('marvel').concat_map(lambda hero: hero['defeatedMonsters']).run(conn)
```
