---
layout: api-command
language: JavaScript
permalink: api/javascript/outer_join/
command: outerJoin
io:
    -   - sequence
        - stream
    -   - array
        - array
related_commands:
    innerJoin: inner_join/
    eqJoin: eq_join/
    zip: zip/
---

# Command syntax #

{% apibody %}
sequence.outerJoin(otherSequence, predicate) &rarr; stream
array.outerJoin(otherSequence, predicate) &rarr; array
{% endapibody %}

# Description #

Returns the outer product of two sequences (e.g. a table, a filter result). The query returns each row of the left sequence paired with each row of the right sequence that satisfies the predicate. The predicate can be either a field name to join on or a function that filters the input sequence.

Note that `outerJoin` is slower and much less efficient than using [concatMap](/api/javascript/concat_map/) with [getAll](/api/javascript/get_all). You should avoid using `outerJoin` in commands when possible.

__Example:__ Construct a sequence of documents containing all cross-universe matchups
where a Marvel hero would lose, but keep Marvel heroes who would never lose a matchup in
the sequence.

```js
r.table('marvel').outerJoin(r.table('dc'), function(marvelRow, dcRow) {
    return marvelRow('strength').lt(dcRow('strength'))
}).run(conn, callback)
```

__Example:__ Join a sequence on a simple field name.

```js
r.table('players').outerJoin(r.table('games'), 'game_id').run(conn, callback)
```

__Example:__ Use [nested field](/docs/cookbook/javascript/#filtering-based-on-nested-fields) syntax to join on fields from subdocuments.

```js
r.table('players').outerJoin(r.table('games'), r.row('game')('id')).run(conn, callback)
```
