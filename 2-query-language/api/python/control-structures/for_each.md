---
layout: api-command 
language: Python
permalink: api/python/for_each/
command: for_each 
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/python/control-structures/for_each.md
---

# Command syntax #

{% apibody %}
sequence.for_each(write_query) &rarr; object
{% endapibody %}

# Description #

Loop over a sequence, evaluating the given write query for each element.

__Example:__ Now that our heroes have defeated their villains, we can safely remove them from the villain table.

```py
r.table('marvel').for_each(
    lambda hero: r.table('villains').get(hero['villainDefeated']).delete()
).run(conn)
```



