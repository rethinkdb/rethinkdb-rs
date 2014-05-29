---
layout: api-command
language: Ruby
permalink: api/ruby/nth/
command: 'nth, []'
related_commands:
    order_by: order_by/
    skip: skip/
    limit: limit/
    'slice, []': slice/
---

# Command syntax #

{% apibody %}
sequence.nth(index) &rarr; object
selection.nth(index) &rarr; selection&lt;object&gt;
{% endapibody %}

# Description #

Get the *nth* element of a sequence.

You may also use Ruby's array syntax, `[]`, as a shorthand for `nth`.

__Example:__ Select the second element in the array.

```rb
r.expr([1,2,3]).nth(1).run(conn)
r.expr([1,2,3])[1].run(conn)
```

**Example:** Select the bronze medalist from the competitors.

```rb
r.table('players').order_by(:index => r.desc('score')).nth(3).run(conn)
```
