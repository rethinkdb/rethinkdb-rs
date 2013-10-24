---
layout: api-command 
language: Ruby
permalink: api/ruby/sample/
command: sample 
---

# Command syntax #

{% apibody %}
sequence.sample(number) &rarr; selection
stream.sample(number) &rarr; array
array.sample(number) &rarr; array
{% endapibody %}

# Description #

Select a given number of elements from a sequence with uniform random distribution. Selection is done without replacement.

__Example:__ Select 3 random heroes.

```rb
r.table('marvel').sample(3).run(conn)
```
