---
layout: api-command
language: Ruby
permalink: api/ruby/map/
command: map
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

```rb
r.table('marvel').map {|hero|
    hero[:combat_power] + hero[:compassion_power] * 2
}.run(conn)
```


