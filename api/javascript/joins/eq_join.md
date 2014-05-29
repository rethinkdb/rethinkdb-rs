---
layout: api-command
language: JavaScript
permalink: api/javascript/eq_join/
command: eqJoin
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

# Command syntax #

{% apibody %}
sequence.eqJoin(leftAttr, otherTable[, {index:'id'}]) &rarr; stream
array.eqJoin(leftAttr, otherTable[, {index:'id'}]) &rarr; array
{% endapibody %}

# Description #

Join tables using an attribute on the left table matching primary keys or secondary indexes on the right table.

Documents in the right table missing the joined attribute or that have that attribute present with a value of `null` will not be returned by `eqJoin`.

**Example:** Look up all of a player's matches.

```js
r.table('players').eqJoin('game_id', r.table('games')).run(conn, callback)
```

This is equivalent to the following `innerJoin,` but runs in *O(n &times; log(m))* time rather than *O( n &times; m)* time.

```js
r.table('players').innerJoin(r.table('games'), function(left, right) {
    return left('game_id').eq(right('id'));
}).run(conn, callback)
```

**Example:** Use a secondary index on the right table rather than the primary key. If players have a secondary index on their cities, we can get a list of arenas with players in the same area.

```js
r.table('arenas').eqJoin('city_id', r.table('arenas'), {index: 'city_id'}).run(conn, callback)
```

**Example:** Use a function instead of an attribute to join on a more complicated expression.

```js
r.table('players').eqJoin(function (player) {
    return player('bestScores').orderBy({index: r.desc('scores')}).limit(3)('game_id');
}, r.table('games')).run(conn, callback)
```
