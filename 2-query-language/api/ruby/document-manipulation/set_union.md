---
layout: api-command 
language: Ruby
permalink: api/ruby/set_union/
command: set_union 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/document-manipulation/set_union.md
related_commands:
    union: union/
    difference: difference/
    set_insert: set_insert/
    set_intersection: set_intersection/
    set_difference: set_difference/
---

# Command syntax #

{% apibody %}
array.set_union(array) &rarr; array
{% endapibody %}

# Description #

Add a several values to an array and return it as a set (an array with distinct values).

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots and an arc reactor.

```rb
r.table('marvel').get('IronMan')[:equipment].set_union(['newBoots', 'arc_reactor']).run(conn)
```


