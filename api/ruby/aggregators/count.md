---
layout: api-command
language: Ruby
permalink: api/ruby/count-aggregator/
command: count
related_commands:
    group_by: group_by
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

```rb
r.table('marvel').group_by(:strength, r.count).run(conn)
```


