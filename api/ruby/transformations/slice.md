---
layout: api-command 
language: Ruby
permalink: api/ruby/slice/
command: '[] (slice)'
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/transformations/slice.md
related_commands:
    order_by: order_by/
    skip: skip/
    '[] (nth)': nth/
---

# Command syntax #

{% apibody %}
sequence[start_index[, end_index]] &rarr; stream
array[start_index[, end_index]] &rarr; array
{% endapibody %}

# Description #

Trim the sequence to within the bounds provided.

__Example:__ For this fight, we need heroes with a good mix of strength and agility.

```rb
r.table('marvel').order_by(:strength)[5..10].run(conn)
```


