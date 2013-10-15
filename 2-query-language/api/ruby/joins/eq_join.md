---
layout: api-command 
language: Ruby
permalink: api/ruby/eq_join/
command: eq_join 
---


{% apibody %}
sequence.eq_join(left_attr, other_table[, index='id']) → stream
array.eq_join(left_attr, other_table[, index='id']) → array
{% endapibody %}

An efficient join that looks up elements in the right table by primary key.

__Example:__ Let our heroes join forces to battle evil!

```rb
r.table('marvel').eq_join(:main_dc_collaborator, r.table('dc')).run(conn)
```

__Example:__ The above query is equivalent to this inner join but runs in O(n log(m))
time rather than the O(n * m) time the inner join takes.

```rb
r.table('marvel').inner_join(r.table('dc')) {|left, right|
    left[:main_dc_collaborator].eq(right[:hero_name])
}.run(conn)
```


__Example:__ You can take advantage of a secondary index on the second table by giving
an optional index parameter.

```rb
r.table('marvel').eq_join('main_weapon_origin',
r.table('mythical_weapons'), :index => 'origin').run(conn)
```


__Example:__ You can pass a function instead of an attribute to join on more
complicated expressions. Here we join to the DC universe collaborator with whom the
hero has the most appearances.

```rb
r.table('marvel').eq_join(lambda {|doc| doc[:dc_collaborators].order_by(:appearances)[0][:name]},
r.table('dc')).run(conn)
```

