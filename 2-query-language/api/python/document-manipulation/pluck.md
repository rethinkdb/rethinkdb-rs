---
layout: api-command 
language: Python
permalink: api/python/pluck/
command: pluck
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/document-manipulation/pluck.md
---

{% apibody %}
sequence.pluck([selector1, selector2...]) → stream
array.pluck([selector1, selector2...]) → array
object.pluck([selector1, selector2...]) → object
singleSelection.pluck([selector1, selector2...]) → object
{% endapibody %}

Plucks out one or more attributes from either an object or a sequence of objects
(projection).

__Example:__ We just need information about IronMan's reactor and not the rest of the
document.

```py
r.table('marvel').get('IronMan').pluck('reactorState', 'reactorPower').run(conn)
```


__Example:__ For the hero beauty contest we only care about certain qualities.

```py
r.table('marvel').pluck('beauty', 'muscleTone', 'charm').run(conn)
```


__Example:__ Pluck can also be used on nested objects.

```py
r.table('marvel').pluck({'abilities' : {'damage' : True, 'mana_cost' : True}, 'weapons' : True}).run(conn)
```


__Example:__ The nested syntax can quickly become overly verbose so there's a shorthand
for it.

```py
r.table('marvel').pluck({'abilities' : ['damage', 'mana_cost']}, 'weapons').run(conn)
```

