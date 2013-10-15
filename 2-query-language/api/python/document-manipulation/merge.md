---
layout: api-command 
language: Python
permalink: api/python/merge/
command: merge 
---

{% apibody %}
singleSelection.merge(object) → object
object.merge(object) → object
sequence.merge(object) → stream
array.merge(object) → array
{% endapibody %}

Merge two objects together to construct a new object with properties from both. Gives preference to attributes from other when there is a conflict.

__Example:__ Equip IronMan for battle.

```py
r.table('marvel').get('IronMan').merge(
    r.table('loadouts').get('alienInvasionKit')
).run(conn)
```

__Example:__ Merge can be used recursively to modify object within objects.

```py
r.expr({'weapons' : {'spectacular graviton beam' : {'dmg' : 10, 'cooldown' : 20}}}).merge(
    {'weapons' : {'spectacular graviton beam' : {'dmg' : 10}}}
).run(conn)
```


__Example:__ To replace a nested object with another object you can use the literal keyword.

```py
r.expr({'weapons' : {'spectacular graviton beam' : {'dmg' : 10, 'cooldown' : 20}}}).merge(
    {'weapons' : r.literal({'repulsor rays' : {'dmg' : 3, 'cooldown' : 0}})}
).run(conn)
```


__Example:__ Literal can be used to remove keys from an object as well.

```py
r.expr({'weapons' : {'spectacular graviton beam' : {'dmg' : 10, 'cooldown' : 20}}}).merge(
    {'weapons' : {'spectacular graviton beam' : r.literal()}}
).run(conn)
```

