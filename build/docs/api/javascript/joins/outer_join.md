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
sequence.outerJoin(otherSequence, predicate_function) &rarr; stream
array.outerJoin(otherSequence, predicate_function) &rarr; array
{% endapibody %}

# Description #

Returns a left outer join of two sequences. The returned sequence represents a union of the left-hand sequence and the right-hand sequence: all documents in the left-hand sequence will be returned, each matched with a document in the right-hand sequence if one satisfies the predicate condition. In most cases, you will want to follow the join with [zip](/api/javascript/zip) to combine the left and right results.


{% infobox %}
Note that `outerJoin` is slower and much less efficient than using [concatMap](/api/javascript/concat_map/) with [getAll](/api/javascript/get_all). You should avoid using `outerJoin` in commands when possible.
{% endinfobox %}

__Example:__ Return a list of all Marvel heroes, paired with any DC heroes who could beat them in a fight.

```js
r.table('marvel').outerJoin(r.table('dc'), function(marvelRow, dcRow) {
    return marvelRow('strength').lt(dcRow('strength'))
}).run(conn, callback)
```

(Compare this to an [innerJoin](/api/javascript/inner_join) with the same inputs and predicate, which would return a list only of the matchups in which the DC hero has the higher strength.)