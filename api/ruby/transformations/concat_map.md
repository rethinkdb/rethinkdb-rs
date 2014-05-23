---
layout: api-command
language: Ruby
permalink: api/ruby/concat_map/
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

```rb
r.expr([1, 2, 3]).map { |x| [x, x.mul(2)] }.run(conn)
```

Result:

```js
[[1, 2], [2, 4], [3, 6]]
```

Whereas `concat_map` with the same mapping function would merge those sequences into one:

```rb
r.expr([1, 2, 3]).concat_map { |x| [x, x.mul(2)] }.run(conn)
```

Result:

```js
[1, 2, 2, 4, 3, 6]
```

The return value, array or stream, will be the same type as the input.

__Example:__ Construct a sequence of all monsters defeated by Marvel heroes. The field "defeatedMonsters" is an array of one or more monster names.

```rb
r.table('marvel').concat_map { |hero|
    hero[:defeated_monsters]
}.run(conn)
```

__Example:__ Simulate an [eq_join](/api/ruby/eq_join/) using `concat_map`. (This is how ReQL joins are implemented internally.)

```rb
r.table('posts').concat_map { |post|
    r.table('comments').get_all(
        post['id'], :index => 'post_id'
    ).map { |comment| { :left => post, :right => comment} }
}.run(conn)
```