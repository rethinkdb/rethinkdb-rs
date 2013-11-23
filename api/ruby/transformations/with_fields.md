---
layout: api-command
language: Ruby
permalink: api/ruby/with_fields/
command: with_fields
related_commands:
    has_fields: has_fields/
    pluck: pluck/
    without: without/
---

# Command syntax #

{% apibody %}
sequence.with_selectors([selector1, selector2...]) &rarr; stream
array.with_selectors([selector1, selector2...]) &rarr; array
{% endapibody %}

# Description #

Takes a sequence of objects and a list of fields. If any objects in the sequence don't
have all of the specified fields, they're dropped from the sequence. The remaining
objects have the specified fields plucked out. (This is identical to `has_fields`
followed by `pluck` on a sequence.)

__Example:__ Get a list of heroes and their nemeses, excluding any heroes that lack one.

```rb
r.table('marvel').with_fields('id', 'nemesis')
```

__Example:__ Get a list of heroes and their nemeses, excluding any heroes whose nemesis
isn't in an evil organization.

```rb
r.table('marvel').with_fields(:id, {:nemesis => {:evil_organization => true}})
```


__Example:__ The nested syntax can quickly become overly verbose so there's a shorthand.

```rb
r.table('marvel').with_fields(:id, {:nemesis => :evil_organization})
```

