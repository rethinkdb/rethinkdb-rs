---
layout: api-command
language: JavaScript
permalink: api/javascript/distinct/
command: distinct
io:
    -   - sequence
        - value
related_commands:
    map: map/
    concat_map: concat_map/
    group: group/
---

# Command syntax #

{% apibody %}
sequence.distinct() &rarr; array
{% endapibody %}

# Description #

Removes duplicate elements from a sequence.  Returns an array even
when called on a stream.  Meant for use on small sequences.

__Example:__ Which unique villains have been vanquished by marvel heroes?

```js
r.table('marvel').concatMap(function(hero) {
    return hero('villainList')
}).distinct().run(conn, callback)
```
