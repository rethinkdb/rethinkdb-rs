---
layout: api-command
language: Ruby
permalink: api/ruby/merge/
command: merge
related_commands:
    pluck: pluck/
    without: without/
    map: map/
---

# Command syntax #

{% apibody %}
singleSelection.merge(object) &rarr; object
object.merge(object) &rarr; object
sequence.merge(object) &rarr; stream
array.merge(object) &rarr; array
{% endapibody %}

# Description #

Merge two objects together to construct a new object with properties from both. Gives preference to attributes from other when there is a conflict.

__Example:__ Equip IronMan for battle.

```rb
r.table('marvel').get('IronMan').merge(
    r.table('loadouts').get('alienInvasionKit')
).run(conn)
```

__Example:__ Merge can be used recursively to modify object within objects.

```rb
r.expr({:weapons => {:spectacular_graviton_beam => {:dmg => 10, :cooldown => 20}}}).merge(
    {:weapons => {:spectacular_graviton_beam => {:dmg => 10}}}
).run(conn)
```

__Example:__ To replace a nested object with another object you can use the literal keyword.

```
r.expr({:weapons => {:spectacular_graviton_beam => {:dmg => 10, :cooldown => 20}}}).merge(
    {:weapons => r.literal({:repulsor_rays => {:dmg => 3, :cooldown => 0}})}
).run(conn)
```


__Example:__ Literal can be used to remove keys from an object as well.

```rb
r.expr({:weapons => {:spectacular_graviton_beam => {:dmg => 10, :cooldown => 20}}}).merge(
    {:weapons => {:spectacular_graviton_beam => r.literal()}}
).run(conn)
```

