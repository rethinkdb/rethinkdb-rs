---
layout: api-command
language: JavaScript
permalink: api/javascript/count-aggregator/
command: count
io:
    -   - r
        - null
related_commands:
    groupBy: group_by/
    sum: sum/
    avg: avg/
---

# Command syntax #

{% apibody %}
r.count
{% endapibody %}

# Description #

Count the total size of the group.

__Example:__ Just how many heroes do we have at each strength level?

```js
r.table('marvel').groupBy('strength', r.count).run(conn, callback)
```

