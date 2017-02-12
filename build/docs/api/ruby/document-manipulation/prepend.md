---
layout: api-command
language: Ruby
permalink: api/ruby/prepend/
command: prepend
related_commands:
---

# Command syntax #

{% apibody %}
array.prepend(value) &rarr; array
{% endapibody %}

# Description #

Prepend a value to an array.

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```rb
r.table('marvel').get('IronMan')[:equipment].prepend('new_boots').run(conn)
```


