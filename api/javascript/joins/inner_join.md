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
sequence.innerJoin(otherSequence, predicate_function) &rarr; stream
array.innerJoin(otherSequence, predicate_function) &rarr; array
{% endapibody %}

# Description #

Returns an inner join of two sequences.

The returned sequence represents an intersection of the left-hand sequence and the right-hand sequence: each row of the left-hand sequence will be compared with each row of the right-hand sequence to find all pairs of rows which satisfy the predicate. Each matched pair of rows of both sequences are combined into a result row. In most cases, you will want to follow the join with [zip](/api/javascript/zip) to combine the left and right results.

{% infobox %}
Note that `innerJoin` is slower and much less efficient than using [eqJoin](/api/javascript/eq_join/) or [concatMap](/api/javascript/concat_map/) with [getAll](/api/javascript/get_all/). You should avoid using `innerJoin` in commands when possible.
{% endinfobox %}

__Example:__ Return a list of all matchups between Marvel and DC heroes in which the DC hero could beat the Marvel hero in a fight.

```javascript
r.table('marvel').innerJoin(r.table('dc'), function(marvelRow, dcRow) {
    return marvelRow('strength').lt(dcRow('strength'))
}).zip().run(conn, callback)
```

<!-- stop -->

(Compare this to an [outerJoin](/api/javascript/outer_join) with the same inputs and predicate, which would return a list of *all* Marvel heroes along with any DC heroes with a higher strength.)