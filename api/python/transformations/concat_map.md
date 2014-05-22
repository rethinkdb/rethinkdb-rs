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

Concatenate one or more elements into a single sequence using a mapping function.

`concat_map` works in a similar fashion to `map`, applying the given function to each element in a sequence, but it will always return a single sequence. If the mapping function returns a sequence, `map` would produce a sequence of sequences:

    [[1, 2], [3, 4], [5, 6]]

Whereas `concat_map` with the same mapping function would merge those sequences:

    [1, 2, 3, 4, 5, 6]

The return value, array or stream, will be the same type as the input.

__Example:__ Construct a sequence of all monsters defeated by Marvel heroes. The field "defeatedMonsters" is an array of one or more monster names.

```py
r.table('marvel').concat_map(lambda hero: hero['defeatedMonsters']).run(conn)
```
