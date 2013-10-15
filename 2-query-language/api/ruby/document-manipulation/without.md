---
layout: api-command 
language: Ruby
permalink: api/ruby/without/
command: without 
---


{% apibody %}
sequence.without([selector1, selector2...]) → stream
array.without([selector1, selector2...]) → array
singleSelection.without([selector1, selector2...]) → object
object.without([selector1, selector2...]) → object
{% endapibody %}

The opposite of pluck; takes an object or a sequence of objects, and returns them with
the specified paths removed.

__Example:__ Since we don't need it for this computation we'll save bandwidth and leave
out the list of IronMan's romantic conquests.

```rb
r.table('marvel').get('IronMan').without('personalVictoriesList').run(conn)
```

__Example:__ Without their prized weapons, our enemies will quickly be vanquished.

```rb
r.table('enemies').without('weapons').run(conn)
```


__Example:__ Nested objects can be used to remove the damage subfield from the weapons and abilities fields.

```rb
r.table('marvel').without({:weapons => {:damage => true}, :abilities => {:damage => true}}).run(conn)
```


__Example:__ The nested syntax can quickly become overly verbose so there's a shorthand for it.

```rb
r.table('marvel').without({:weapons => :damage, :abilities => :damage}).run(conn)
```

