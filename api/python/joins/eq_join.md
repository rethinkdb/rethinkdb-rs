---
layout: api-command 
language: Python
permalink: api/python/eq_join/
command: eq_join
related_commands:
    inner_join: inner_join/
    outer_join: outer_join/
    zip: zip/
---

# Command syntax #

{% apibody %}
sequence.eq_join(left_attr, other_table[, index='id']) &rarr; stream
array.eq_join(left_attr, other_table[, index='id']) &rarr; array
{% endapibody %}

# Description #

An efficient join that looks up elements in the right table by primary key.

__Example:__ Let our heroes join forces to battle evil!

```py
r.table('marvel').eq_join('main_dc_collaborator', r.table('dc')).run(conn)
```


__Example:__ The above query is equivalent to this inner join but runs in O(n log(m))
time rather than the O(n * m) time the inner join takes.

```py
r.table('marvel').inner_join(r.table('dc'),
lambda left, right: left['main_dc_collaborator'] == right['hero_name']).run(conn)
```


__Example:__ You can take advantage of a secondary index on the second table by giving
an optional index parameter.

```py
r.table('marvel').eq_join('main_weapon_origin',
r.table('mythical_weapons'), index='origin').run(conn)
```


__Example:__ You can pass a function instead of an attribute to join on more
complicated expressions. Here we join to the DC universe collaborator with whom the hero
has the most appearances.

```py
r.table('marvel').eq_join(lambda doc:
    doc['dc_collaborators'].order_by('appearances')[0]['name'],
    r.table('dc')).run(conn)
```
