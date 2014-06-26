---
layout: api-command
language: JavaScript
permalink: api/javascript/inner_join/
command: innerJoin
io:
    -   - sequence
        - stream
    -   - array
        - array
related_commands:
    outerJoin: outer_join/
    eqJoin: eq_join/
    zip: zip/
---

# Command syntax #

{% apibody %}
sequence.innerJoin(otherSequence, predicate) &rarr; stream
array.innerJoin(otherSequence, predicate) &rarr; array
{% endapibody %}

# Description #

Returns the inner product of two sequences (e.g. a table, a filter result) filtered by the predicate. The query compares each row of the left sequence with each row of the right sequence to find all pairs of rows which satisfy the predicate. When the predicate is satisfied, each matched pair of rows of both sequences are combined into a result row. The predicate can be either a field name to join on or a function that filters the input sequence. In most cases, you will want to follow the join with [zip](/api/javascript/zip) to combine the left and right results.

Note that `innerJoin` is slower and much less efficient than using [eqJoin](/api/javascript/eq_join/) or [concatMap](/api/javascript/concat_map/) with [getAll](/api/javascript/get_all/). You should avoid using `innerJoin` in commands when possible.

__Example:__ Construct a sequence of documents containing all cross-universe matchups where a Marvel hero would lose.

```js
r.table('marvel').innerJoin(r.table('dc'), function(marvelRow, dcRow) {
    return marvelRow('strength').lt(dcRow('strength'))
}).zip().run(conn, callback)
```

__Example:__ Join a sequence on a simple field name.

```js
r.table('players').innerJoin(r.table('games'), 'game_id').zip().run(conn, callback)
```
