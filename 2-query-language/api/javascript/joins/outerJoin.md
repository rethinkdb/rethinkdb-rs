---
layout: api-command 
language: JavaScript
permalink: api/javascript/outer_join/
command: outerJoin
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/joins/outerJoin.md
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

{% apibody %}
sequence.outerJoin(otherSequence, predicate) &rarr; stream
array.outerJoin(otherSequence, predicate) &rarr; array
{% endapibody %}

Computes a left outer join by retaining each row in the left table even if no match was
found in the right table.

__Example:__ Construct a sequence of documents containing all cross-universe matchups
where a marvel hero would lose, but keep marvel heroes who would never lose a matchup in
the sequence.

```js
r.table('marvel').outerJoin(r.table('dc'), function(marvelRow, dcRow) {
    return marvelRow('strength').lt(dcRow('strength'))
}).run(conn, callback)
```
