---
layout: api-command
language: Ruby
permalink: api/ruby/for_each/
command: for_each
---

# Command syntax #

{% apibody %}
sequence.for_each(write_query) &rarr; object
{% endapibody %}

# Description #

Loop over a sequence, evaluating the given write query for each element.

__Example:__ Now that our heroes have defeated their villains, we can safely remove them from the villain table.

```rb
r.table('marvel').for_each {|hero|
    r.table('villains').get(hero[:villain_defeated]).delete
}.run(conn)
```
