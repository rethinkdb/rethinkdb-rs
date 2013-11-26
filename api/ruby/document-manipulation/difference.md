---
layout: api-command
language: Ruby
permalink: api/ruby/difference/
command: difference
related_commands:
---

# Command syntax #

{% apibody %}
array.difference(array) &rarr; array
{% endapibody %}

# Description #

Remove the elements of one array from another array.

__Example:__ Retrieve Iron Man's equipment list without boots.

```rb
r.table('marvel').get('IronMan')[:equipment].difference(['Boots']).run(conn)
```


