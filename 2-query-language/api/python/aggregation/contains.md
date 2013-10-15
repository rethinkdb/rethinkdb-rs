---
layout: api-command 
language: Python
permalink: api/python/contains/
command: contains
---

{% apibody %}
sequence.contains(value1[, value2...]) → bool
{% endapibody %}

Returns whether or not a sequence contains all the specified values, or if functions are
provided instead, returns whether or not a sequence contains values matching all the
specified functions.

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

