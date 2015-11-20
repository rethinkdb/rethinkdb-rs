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
singleSelection.merge([object | function, object | function, ...]) &rarr; object
object.merge([object | function, object | function, ...]) &rarr; object
sequence.merge([object | function, object | function, ...]) &rarr; stream
array.merge([object | function, object | function, ...]) &rarr; array
{% endapibody %}

# Description #

Merge two or more objects together to construct a new object with properties from all. When there is a conflict between field names, preference is given to fields in the rightmost object in the argument list `merge` also accepts a subquery function that returns an object, which will be used similarly to a [map](/api/python/map/) function.

__Example:__ Equip Thor for battle.

```py
r.table('marvel').get('thor').merge(
    r.table('equipment').get('hammer'),
    r.table('equipment').get('pimento_sandwich')
).run(conn)
```

__Example:__ Equip every hero for battle, using a subquery function to retrieve their weapons.

```py
r.table('marvel').merge(lambda hero:
    { 'weapons': r.table('weapons').get(hero['weapon_id']) }
).run(conn)
```

__Example:__ Use `merge` to join each blog post with its comments.

Note that the sequence being merged&mdash;in this example, the comments&mdash;must be coerced from a selection to an array. Without `coerce_to` the operation will throw an error ("Expected type DATUM but found SELECTION").

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

