---
layout: api-command 
language: JavaScript
permalink: api/javascript/without/
command: without
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/document-manipulation/without.md
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

```js
r.table('marvel').get('IronMan').without('personalVictoriesList').run(conn, callback)
```


__Example:__ Without their prized weapons, our enemies will quickly be vanquished.

```js
r.table('enemies').without('weapons').run(conn, callback)
```


__Example:__ Nested objects can be used to remove the damage subfield from the weapons and abilities fields.

```js
r.table('marvel').without({'weapons' : {'damage' : true}, 'abilities' : {'damage' : true}}).run(conn, callback)
```


__Example:__ The nested syntax can quickly become overly verbose so there's a shorthand for it.

```js
r.table('marvel').without({'weapons':'damage', 'abilities':'damage'}).run(conn, callback)
```

