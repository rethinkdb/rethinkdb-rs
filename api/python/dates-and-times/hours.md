---
layout: api-command
language: Python
permalink: api/python/hours/
command: hours
related_commands:
    now: now/
    time: time/
    in_timezone: in_timezone/
---

# Command syntax #

{% apibody %}
time.hours() &rarr; number
{% endapibody %}

# Description #

Return the hour in a time object as a number between 0 and 23.

__Example:__ Return all the posts submitted after midnight and before 4am.

```py
r.table("posts").filter(lambda post:
    post["date"].hours() < 4
).run(conn)
```

