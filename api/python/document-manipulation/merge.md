---
layout: api-command
language: Python
permalink: api/python/merge/
command: merge
related_commands:
    pluck: pluck/
    without: without/
    map: map/
---

# Command syntax #

{% apibody %}
singleSelection.merge(object|function) &rarr; object
object.merge(object|function) &rarr; object
sequence.merge(object|function) &rarr; stream
array.merge(object|function) &rarr; array
{% endapibody %}

# Description #

Merge two objects together to construct a new object with properties from both. Gives preference to attributes from other when there is a conflict. `merge` also accepts a subquery function that returns an object, which will be used similarly to a [map](/api/python/map/) function.

__Example:__ Equip IronMan for battle.

```py
r.table('marvel').get('IronMan').merge(
    r.table('loadouts').get('alienInvasionKit')
).run(conn)
```

__Example:__ Equip every hero for battle, using a subquery function to retrieve their weapons.

```py
r.table('marvel').merge(lambda hero:
    { 'weapons': r.table('weapons').get(hero['weapon_id']) }
).run(conn)
```

__Example:__ Use `merge` to join each blog post with its comments.

```py
r.table('posts').merge(lambda post:
    { 'comments': r.table('comments').get_all(post['id'],
        index='post_id').coerce_to('array') }
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

