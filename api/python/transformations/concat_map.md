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
stream.concat_map(function) &rarr; stream
array.concat_map(function) &rarr; array
{% endapibody %}

# Description #

Concatenate one or more elements into a single sequence using a mapping function.

`concat_map` works in a similar fashion to [map](/api/python/map/), applying the given function to each element in a sequence, but it will always return a single sequence. If the mapping function returns a sequence, `map` would produce a sequence of sequences:

```py
r.expr([1, 2, 3]).map(lambda x: [x, x.mul(2)]).run(conn)
```

Result:

```py
[[1, 2], [2, 4], [3, 6]]
```

Whereas `concat_map` with the same mapping function would merge those sequences into one:

```py
r.expr([1, 2, 3]).concat_map(lambda x: [x, x.mul(2)]).run(conn)
```

Result:

```py
[1, 2, 2, 4, 3, 6]
```

The return value, array or stream, will be the same type as the input.

__Example:__ Construct a sequence of all monsters defeated by Marvel heroes. The field "defeatedMonsters" is an array of one or more monster names.

```py
r.table('marvel').concat_map(lambda hero: hero['defeatedMonsters']).run(conn)
```

__Example:__ Simulate an [eq_join](/api/python/eq_join/) using `concat_map`. (This is how ReQL joins are implemented internally.)

```py
r.table('posts').concat_map(
    lambda post: r.table('comments').get_all(
        post['id'], index='post_id'
    ).map(
        lambda comment: { 'left': post, 'right': comment}
    )
).run(conn)
```
