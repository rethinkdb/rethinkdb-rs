---
layout: api-command
language: Java
permalink: api/javascript/zip/
command: zip
related_commands:
    eqJoin: eq_join/
    innerJoin: inner_join/
    outerJoin: outer_join/
---

# Command syntax #

{% apibody %}
stream.zip() &rarr; stream
array.zip() &rarr; array
{% endapibody %}

# Description #

Used to 'zip' up the result of a join by merging the 'right' fields into 'left' fields of each member of the sequence.

__Example:__ 'zips up' the sequence by merging the left and right fields produced by a join.

```
r.table('marvel').eqJoin('main_dc_collaborator', r.table('dc'))
    .zip().run(conn)
```


