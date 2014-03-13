---
layout: api-command
language: Python
permalink: api/python/contains/
command: contains
related_commands:
    map: map/
    concat_map: concat_map/
    group: group/
---

# Command syntax #

{% apibody %}
sequence.contains(value1[, value2...]) &rarr; bool
sequence.contains(predicate1[, predicate2...]) &rarr; bool
{% endapibody %}

# Description #

When called with values, returns `True` if a sequence contains all the
specified values.  When called with predicate functions, returns `True`
if for each predicate there exists at least one element of the stream
where that predicate returns `True`.


__Example:__ Has Iron Man ever fought Superman?

```py
r.table('marvel').get('ironman')['opponents'].contains('superman').run(conn)
```


__Example:__ Has Iron Man ever defeated Superman in battle?

```py
r.table('marvel').get('ironman')['battles'].contains(lambda battle:
    (battle['winner'] == 'ironman') & (battle['loser'] == 'superman')
).run(conn)
```

