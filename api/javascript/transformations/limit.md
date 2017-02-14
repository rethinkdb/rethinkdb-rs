---
layout: api-command
language: JavaScript
permalink: api/javascript/limit/
command: limit
io:
    -   - sequence
        - stream
    -   - array
        - array
related_commands:
    skip: skip/
    slice: slice/
    nth: nth/
---

# Command syntax #

{% apibody %}
sequence.limit(n) &rarr; stream
array.limit(n) &rarr; array
{% endapibody %}

# Description #


End the sequence after the given number of elements.

__Example:__ Only so many can fit in our Pantheon of heroes.

```javascript
r.table('marvel').orderBy('belovedness').limit(10).run(conn, callback)
```


