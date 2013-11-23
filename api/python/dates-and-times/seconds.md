---
layout: api-command
language: Python
permalink: api/python/seconds/
command: seconds
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.seconds() &rarr; number
{% endapibody %}

# Description #

Return the seconds in a time object as a number between 0 and 59.999 (double precision).

__Example:__ Return the post submitted during the first 30 seconds of every minute.

```py
r.table("posts").filter(lambda post:
    post["date"].seconds() < 30
).run(conn)
```


