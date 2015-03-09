---
layout: api-command
language: Ruby
permalink: api/ruby/bracket/
command: '[] (bracket)'
related_commands:
    nth: nth/
    get_field: get_field/
---

# Command syntax #

{% apibody %}
sequence[attr] &rarr; sequence
singleSelection[attr] &rarr; value
object[attr] &rarr; value
array[index] &rarr; value
{% endapibody %}

# Description #

Get a single field from an object. If called on a sequence, gets that field from every object in the sequence, skipping objects that lack it.

__Example:__ What was Iron Man's first appearance in a comic?

```rb
r.table('marvel').get('IronMan')[:first_appearance].run(conn)
```

The `[]` command also accepts integer arguments as array offsets, like the [nth](/api/ruby/nth) command.

__Example:__ Get the fourth element in a sequence. (The first element is position `0`, so the fourth element is position `3`.)

```rb
r.expr([10, 20, 30, 40, 50])[3]

40
```
