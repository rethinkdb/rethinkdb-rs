---
layout: api-command 
language: Python
permalink: api/python/timezone/
command: timezone 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/dates-and-times/timezone.md
related_commands:
    in_timezone: in_timezone/
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.timezone() &rarr; string
{% endapibody %}

# Description #

Return the timezone of the time object.

__Example:__ Return all the users in the "-07:00" timezone.

```py
r.table("users").filter( lambda user:
    user["subscriptionDate"].timezone() == "-07:00"
)
```


