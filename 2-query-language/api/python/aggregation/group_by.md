---
layout: api-command 
language: Python
permalink: api/python/group_by/
command: group_by
github_doc: https://github.com/rethinkdb/docs/blob/master/2-query-language/api/ruby/aggregation/group_by.md
---

{% apibody %}
sequence.group_by(selector1[, selector2...], reduction_object) &rarr; array
{% endapibody %}

Groups elements by the values of the given attributes and then applies the given
reduction. Though similar to `groupedMapReduce`, `groupBy` takes a standardized object
for specifying the reduction. Can be used with a number of predefined common reductions.

__Example:__ Using a predefined reduction we can easily find the average strength of members of each weight class.

```py
r.table('marvel').group_by('weightClass', r.avg('strength')).run(conn)
```

__Example:__ Groupings can also be specified on nested attributes.

```py
r.table('marvel').group_by({'abilities' : {'primary' : True}}, r.avg('strength')).run(conn)
```


__Example:__ The nested syntax can quickly become verbose so there's a shortcut.

```py
r.table('marvel').group_by({'abilities' : 'primary'}, r.avg('strength')).run(conn)
```

