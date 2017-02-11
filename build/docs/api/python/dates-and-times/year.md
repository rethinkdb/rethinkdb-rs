---
layout: api-command
language: Python
permalink: api/python/year/
command: year
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.year() &rarr; number
{% endapibody %}

# Description #

Return the year of a time object.

__Example:__ Retrieve all the users born in 1986.

```py
r.table("users").filter(lambda user:
    user["birthdate"].year() == 1986
).run(conn)
```


