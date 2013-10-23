---
layout: api-command 
language: JavaScript
permalink: api/javascript/eq_join/
command: eqJoin
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/joins/eqJoin.md
io:
    -   - sequence
        - stream
    -   - array
        - array
related_commands:
    innerJoin: inner_join/
    outerJoin: outer_join/
    zip: zip/
---

{% apibody %}
sequence.eqJoin(leftAttr, otherTable[, {index:'id'}]) → stream
array.eqJoin(leftAttr, otherTable[, {index:'id'}]) → array
{% endapibody %}

An efficient join that looks up elements in the right table by primary key.

__Example:__ Let our heroes join forces to battle evil!

```js
r.table('marvel').eqJoin('main_dc_collaborator', r.table('dc')).run(conn, callback)
```

__Example:__ The above query is equivalent to this inner join but runs in O(n log(m)) time rather than the O(n * m) time the inner join takes.

```js
r.table('marvel').innerJoin(r.table('dc'), function(left, right) {
    return left('main_dc_collaborator').eq(right('hero_name'));
}).run(conn, callback)
```


__Example:__ You can take advantage of a secondary index on the second table by giving an optional index parameter.

```js
r.table('marvel').eqJoin('main_weapon_origin',
r.table('mythical_weapons'), {index:'origin'}).run(conn, callback)
```

__Example:__ You can pass a function instead of an attribute to join on more complicated expressions. Here we join to the DC universe collaborator with whom the hero has the most appearances.

```js
r.table('marvel').eqJoin(function (doc) { return doc('dcCollaborators').orderBy('appearances').nth(0)('name'); },
r.table('dc')).run(conn, callback)
```

