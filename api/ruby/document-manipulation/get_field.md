---
layout: api-command
language: Ruby
permalink: api/ruby/get_field/
command: get_field
related_commands:
    '[] (bracket)': bracket/
    nth: nth/
---

# Command syntax #

{% apibody %}
sequence.get_field(attr) &rarr; sequence
singleSelection.get_field(attr) &rarr; value
object.get_field(attr) &rarr; value
{% endapibody %}

# Description #

Get a single field from an object. If called on a sequence, gets that field from every
object in the sequence, skipping objects that lack it.

__Example:__ What was Iron Man's first appearance in a comic?

```rb
r.table('marvel').get('IronMan').get_field('first_appearance').run(conn)
```
