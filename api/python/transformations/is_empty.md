---
layout: api-command 
language: Python
permalink: api/python/is_empty/
command: is_empty
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/transformations/is_empty.md
related_commands:
    indexes_of: indexes_of/
---

# Command syntax #

{% apibody %}
sequence.is_empty() &rarr; bool
{% endapibody %}

# Description #

Test if a sequence is empty.

__Example:__ Are there any documents in the marvel table?

```py
r.table('marvel').is_empty().run(conn)
```


