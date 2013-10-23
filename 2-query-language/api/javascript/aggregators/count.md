---
layout: api-command 
language: JavaScript
permalink: api/javascript/count-aggregator/
command: count
github_doc: https://github.com/rethinkdb/docs/blob/master/2-query-language/api/javascript/aggregators/count.md
io:
    -   - r
        - null
related_commands:
    groupBy: groupBy/
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

