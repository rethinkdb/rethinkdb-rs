---
layout: api-command 
language: Python
permalink: api/python/to_iso8601/
command: to_iso8601 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/dates-and-times/to_iso8601.md
related_commands:
    now: now/
    time: time/
---

{% apibody %}
time.to_iso8601() &rarr; number
{% endapibody %}

Convert a time object to its iso 8601 format.

__Example:__ Return the current time in an ISO8601 format.

```py
r.now().to_iso8601()
```


