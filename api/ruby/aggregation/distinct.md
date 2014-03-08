---
layout: api-command
language: Ruby
permalink: api/ruby/distinct/
command: distinct
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

```rb
r.table('marvel').concat_map{|hero| hero[:villain_list]}.distinct.run(conn)
```

