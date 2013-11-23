---
layout: api-command
language: Ruby
permalink: api/ruby/distinct/
command: distinct
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

