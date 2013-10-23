---
layout: api-command 
language: Python
permalink: api/python/without/
command: without 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/document-manipulation/without.md
related_commands:
    pluck: pluck/
    map: map/
---

{% apibody %}
sequence.without([selector1, selector2...]) → stream
array.without([selector1, selector2...]) → array
singleSelection.without([selector1, selector2...]) → object
object.without([selector1, selector2...]) → object
{% endapibody %}

The opposite of pluck; takes an object or a sequence of objects, and returns them with
the specified paths removed.

__Example:__ Since we don't need it for this computation we'll save bandwidth and leave
out the list of IronMan's romantic conquests.

```py
r.table('marvel').get('IronMan').without('personalVictoriesList').run(conn)
```


__Example:__ Without their prized weapons, our enemies will quickly be vanquished.

```py
r.table('enemies').without('weapons').run(conn)
```


__Example:__ Nested objects can be used to remove the damage subfield from the weapons and abilities fields.

```py
r.table('marvel').without({'weapons' : {'damage' : True}, 'abilities' : {'damage' : True}}).run(conn)
```


__Example:__ The nested syntax can quickly become overly verbose so there's a shorthand for it.

```py
r.table('marvel').without({'weapons' : 'damage', 'abilities' : 'damage'}).run(conn)
```

