---
layout: api-command 
language: Ruby
permalink: api/ruby/distinct/
command: distinct 
github_doc: https://github.com/rethinkdb/docs/blob/master/2-query-language/api/ruby/aggregation/distinct.md
related_commands:
    map: map/
    concat_map: concat_map/
    grouped_map_reduce: grouped_map_reduce/
---


# Command syntax #

{% apibody %}
sequence.distinct() &rarr; array
{% endapibody %}

# Description #

Remove duplicate elements from the sequence.

__Example:__ Which unique villains have been vanquished by marvel heroes?

```rb
r.table('marvel').concat_map{|hero| hero[:villain_list]}.distinct.run(conn)
```

