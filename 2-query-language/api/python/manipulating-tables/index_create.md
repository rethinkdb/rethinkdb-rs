---
layout: api-command 
language: Python
permalink: api/python/index_create/
command: index_create
---

{% apibody %}
table.index_create(index_name[, index_function]) → object
{% endapibody %}

Create a new secondary index on this table.

__Example:__ To efficiently query our heros by code name we have to create a secondary
index.

```py
r.table('dc').index_create('code_name').run(conn)
```


__Example:__ You can also create a secondary index based on an arbitrary function on the document.

```py
r.table('dc').index_create('power_rating',
lambda hero: hero['combat_power'] + (2 * hero['compassion_power'])
).run(conn)
```


__Example:__ A compound index can be created by returning an array of values to use as
the secondary index key.

```py
r.table('dc').index_create('parental_planets',
lambda hero: [hero['mothers_home_planet'], hero['fathers_home_planet']]
).run(conn)
```


__Example:__ A multi index can be created by passing an optional multi argument. Multi
indexes functions should return arrays and allow you to query based on whether a value
is present in the returned array. The example would allow us to get heroes who possess a
specific ability (the field 'abilities' is an array).

```py
r.table('dc').index_create('abilities', multi=True).run(conn)
```

