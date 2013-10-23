---
layout: api-command 
language: Python
permalink: api/python/concat_map/
command: concat_map 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/transformations/concat_map.md
related_commands:
    map: map/
    reduce: reduce/
---

{% apibody %}
sequence.concat_map(mapping_function) &rarr; stream
array.concat_map(mapping_function) &rarr; array
{% endapibody %}

Flattens a sequence of arrays returned by the mappingFunction into a single sequence.

__Example:__ Construct a sequence of all monsters defeated by Marvel heroes. Here the field
'defeatedMonsters' is a list that is concatenated to the sequence.

```py
r.table('marvel').concat_map(lambda hero: hero['defeatedMonsters']).run(conn)
```
