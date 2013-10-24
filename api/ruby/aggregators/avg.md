---
layout: api-command 
language: Ruby
permalink: api/ruby/avg/
command: avg
github_doc: https://github.com/rethinkdb/docs/blob/master/2-query-language/api/ruby/aggregators/avg.md
related_commands:
    group_by: group_by
    count: count/
    sum: sum/
---

# Command syntax #

{% apibody %}
r.avg(attr)
{% endapibody %}

# Description #

Compute the average value of the given attribute for the group.

__Example:__ What's the average agility of heroes at each strength level?

```rb
r.table('marvel').group_by(:strength, r.avg(:agility)).run(conn)
```


