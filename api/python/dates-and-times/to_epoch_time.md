---
layout: api-command 
language: Python
permalink: api/python/to_epoch_time/
command: to_epoch_time
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/dates-and-times/to_epoch_time.md
related_commands:
    now: now/
    time: time/
    to_iso8601: to_iso8601/
---

# Command syntax #

{% apibody %}
time.to_epoch_time() &rarr; number
{% endapibody %}

# Description #

Convert a time object to its epoch time.

__Example:__ Return the current time in an ISO8601 format.

```py
r.now().to_epoch_time()
```


