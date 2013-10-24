---
layout: api-command 
language: Ruby
permalink: api/ruby/group_by/
command: group_by 
related_commands:
    count: count-aggregation/
    sum: sum
    avg: avg
---

# Command syntax #

{% apibody %}
sequence.group_by(selector1[, selector2...], reduction_object) &rarr; array
{% endapibody %}

# Description #

Groups elements by the values of the given attributes and then applies the given
reduction. Though similar to `groupedMapReduce`, `groupBy` takes a standardized object
for specifying the reduction. Can be used with a number of predefined common reductions.


__Example:__ Using a predefined reduction we can easily find the average strength of members of each weight class.

```rb
r.table('marvel').group_by(:weight_class, r.avg(:strength)).run(conn)
```


__Example:__ Groupings can also be specified on nested attributes.

```rb
r.table('marvel').group_by({:abilities => {:primary => true}}, r.avg(:strength)).run(conn)
```


__Example:__ The nested syntax can quickly become verbose so there's a shortcut.

```rb
r.table('marvel').group_by({:abilities => :primary}, r.avg(:strength)).run(conn)
```

