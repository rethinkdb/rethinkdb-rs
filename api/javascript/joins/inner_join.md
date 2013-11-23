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

Returns the inner product of two sequences (e.g. a table, a filter result) filtered by
the predicate. The query compares each row of the left sequence with each row of the
right sequence to find all pairs of rows which satisfy the predicate. When the predicate
is satisfied, each matched pair of rows of both sequences are combined into a result row.

__Example:__ Construct a sequence of documents containing all cross-universe matchups where a marvel hero would lose.

```js
r.table('marvel').innerJoin(r.table('dc'), function(marvelRow, dcRow) {
    return marvelRow('strength').lt(dcRow('strength'))
}).run(conn, callback)
```

