---
layout: api-command 
language: JavaScript
permalink: api/javascript/merge/
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

```js
r.table('marvel').get('IronMan').merge(
    r.table('loadouts').get('alienInvasionKit')
).run(conn, callback)
```


__Example:__ Merge can be used recursively to modify object within objects.

```js
r.expr({weapons : {spectacular_graviton_beam : {dmg : 10, cooldown : 20}}}).merge(
    {weapons : {spectacular_graviton_beam : {dmg : 10}}}).run(conn, callback)
```


__Example:__ To replace a nested object with another object you can use the literal keyword.

```js
r.expr({weapons : {spectacular_graviton_beam : {dmg : 10, cooldown : 20}}}).merge(
    {weapons : r.literal({repulsor_rays : {dmg : 3, cooldown : 0}})}).run(conn, callback)
```


__Example:__ Literal can be used to remove keys from an object as well.

```js
r.expr({weapons : {spectacular_graviton_beam : {dmg : 10, cooldown : 20}}}).merge(
    {weapons : {spectacular_graviton_beam : r.literal()}}).run(conn, callback)
```

