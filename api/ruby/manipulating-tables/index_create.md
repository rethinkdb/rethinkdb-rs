---
layout: api-command
language: Ruby
permalink: api/ruby/index_create/
command: index_create
related_commands:
    index_list: index_list/
    index_drop: index_drop/
---

# Command syntax #

{% apibody %}
table.index_create(index_name[, index_function]) &rarr; object
{% endapibody %}

# Description #

Create a new secondary index on this table.

__Example:__ To efficiently query our heros by name we can create a secondary index
based on the value of that field. We can already quickly query heros by name with the
primary index but to do the same based on hero code names we'll have to create a
secondary index based on that attribute.

```rb
r.table('dc').index_create('code_name').run(conn)
```


__Example:__ You can also create a secondary index based on an arbitrary function on
the document.

```rb
r.table('dc').index_create('power_rating') {|hero|
    hero['combat_power'] + (2 * hero['compassion_power'])
}.run(conn)
```


__Example:__ A compound index can be created by returning an array of values to use as
the secondary index key.

```rb
r.table('dc').index_create('parental_planets') {|hero|
    [hero['mothers_home_planet'], hero['fathers_home_planet']]
}.run(conn)
```


__Example:__ A multi index can be created by passing an optional multi argument. Multi
index functions should return arrays and allow you to query based on whether a value
is present in the returned array. The example would allow us to get heroes who possess
a specific ability (the field 'abilities' is an array).


```rb
r.table('dc').index_create('abilities', :multi => true).run(conn)
```

__Example:__ The above can be combined to create a multi index on a function that
returns an array of values.

```rb
r.table('dc').index_create('parental_planets', :multi => true) {|hero|
    [hero['mothers_home_planet'], hero['fathers_home_planet']]
}.run(conn)
```
