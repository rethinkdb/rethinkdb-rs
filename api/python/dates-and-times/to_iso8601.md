---
layout: api-command
language: Python
permalink: api/python/to_iso8601/
command: to_iso8601
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.to_iso8601() &rarr; number
{% endapibody %}

# Description #

Convert a time object to its iso 8601 format.

__Example:__ Return the current time in an ISO8601 format.

```py
r.now().to_iso8601()
```


