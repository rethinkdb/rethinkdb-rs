---
layout: api-command 
language: JavaScript
permalink: api/javascript/pluck/
command: pluck
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/document-manipulation/pluck.md
io:
    -   - sequence
        - stream
    -   - array
        - array
    -   - object
        - object
    -   - singleSelection
        - object
related_commands:
    without: without/
    map: map/
---

{% apibody %}
sequence.pluck([selector1, selector2...]) &rarr; stream
array.pluck([selector1, selector2...]) &rarr; array
object.pluck([selector1, selector2...]) &rarr; object
singleSelection.pluck([selector1, selector2...]) &rarr; object
{% endapibody %}

Plucks out one or more attributes from either an object or a sequence of objects
(projection).

__Example:__ We just need information about IronMan's reactor and not the rest of the
document.

```js
r.table('marvel').get('IronMan').pluck('reactorState', 'reactorPower').run(conn, callback)
```


__Example:__ For the hero beauty contest we only care about certain qualities.

```js
r.table('marvel').pluck('beauty', 'muscleTone', 'charm').run(conn, callback)
```


__Example:__ Pluck can also be used on nested objects.

```js
r.table('marvel').pluck({'abilities' : {'damage' : true, 'mana_cost' : true}, 'weapons' : true}).run(conn, callback)
```


__Example:__ The nested syntax can quickly become overly verbose so there's a shorthand for it.

```js
r.table('marvel').pluck({'abilities' : ['damage', 'mana_cost']}, 'weapons').run(conn, callback)
```
