---
layout: api-command 
language: Ruby
permalink: api/ruby/concat_map/
command: concat_map 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/transformations/concat_map.md
related_commands:
    map: map/
    reduce: reduce/
---

# Command syntax #

{% apibody %}
sequence.concat_map(mapping_function) &rarr; stream
array.concat_map(mapping_function) &rarr; array
{% endapibody %}

# Description #

Flattens a sequence of arrays returned by the mappingFunction into a single sequence.

__Example:__ Construct a sequence of all monsters defeated by Marvel heroes. Here the field
'defeatedMonsters' is a list that is concatenated to the sequence.

```rb
r.table('marvel').concat_map {|hero|
    hero[:defeated_monsters]
}.run(conn)

```


