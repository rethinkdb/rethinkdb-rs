---
layout: api-command 
language: Python
permalink: api/python/map/
command: map 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/transformations/map.md
related_commands:
    concat_map: concat_map/
    reduce: reduce/
---

# Command syntax #

{% apibody %}
sequence.map(mapping_function) &rarr; stream
array.map(mapping_function) &rarr; array
{% endapibody %}

# Description #

Transform each element of the sequence by applying the given mapping function.

__Example:__ Construct a sequence of hero power ratings.

```py
r.table('marvel').map( lambda hero:
    hero['combatPower'] + hero['compassionPower'] * 2
).run(conn)
```


