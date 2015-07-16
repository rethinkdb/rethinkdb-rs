---
layout: api-command
language: Ruby
permalink: api/ruby/map/
command: map
related_commands:
    concat_map: concat_map/
    reduce: reduce/
    do: do/
---

# Command syntax #

{% apibody %}
sequence1.map([sequence2, ...], mapping_function) &rarr; stream
array1.map([array2, ...], mapping_function) &rarr; array
r.map(sequence1[, sequence2, ...], mapping_function) &rarr; stream
r.map(array1[, array2, ...], mapping_function) &rarr; array
{% endapibody %}

# Description #

Transform each element of one or more sequences by applying a mapping function to them. If `map` is run with two or more sequences, it will iterate for as many items as there are in the shortest sequence.

Note that `map` can only be applied to sequences, not single values. If you wish to apply a function to a single value/selection (including an array), use the [do](/api/ruby/do) command.

__Example:__ Return the first five squares.

```rb
> r.expr([1, 2, 3, 4, 5]).map{ |val| (val * val) }.run(conn)

[1, 4, 9, 16, 25]
```

__Example:__ Sum the elements of three sequences.

```rb
> sequence1 = [100, 200, 300, 400]
> sequence2 = [10, 20, 30, 40]
> sequence3 = [1, 2, 3, 4]
> r.map(sequence1, sequence2, sequence3){ |val1, val2, val3|
      (val1 + val2 + val3)
  }.run(conn)

[111, 222, 333, 444]
```

__Example:__ Rename a field when retrieving documents using `map` and [merge](/api/ruby/merge/).

This example renames the field `id` to `user_id` when retrieving documents from the table `users`.

```rb
r.table('users').map{ |doc|
    doc.merge({:user_id => doc['id']}).without('id')
}.run(conn)
```

__Example:__ Assign every superhero an archenemy.

```rb
r.table('heroes').map(r.table('villains')){ |hero, villain|
    hero.merge({:villain => villain})
}.run(conn)
```
