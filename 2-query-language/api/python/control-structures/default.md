---
layout: api-command 
language: Python
permalink: api/python/default/
command: default 
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/python/control-structures/default.md
---

{% apibody %}
value.default(default_value) → any
sequence.default(default_value) → any
{% endapibody %}

Handle non-existence errors. Tries to evaluate and return its first argument. If an
error related to the absence of a value is thrown in the process, or if its first
argument returns null, returns its second argument. (Alternatively, the second argument
may be a function which will be called with either the text of the non-existence error
or null.)

__Example:__ Stark Industries made the mistake of trusting an intern with data entry,
and now a bunch of fields are missing from some of their documents. Iron Man takes a
break from fighting Mandarin to write some safe analytics queries.

```py
r.table('projects').map(
    lambda p: p['staff'].default(0) + p['management'].default(0)
).run(conn)
```


