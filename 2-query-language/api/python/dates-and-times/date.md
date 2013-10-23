---
layout: api-command 
language: Python
permalink: api/python/date/
command: date
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/dates-and-times/date.md
related_commands:
    now: now/
    time: time/
    in_timezone: in_timezone/
---

{% apibody %}
time.date() &rarr; time
{% endapibody %}

Return a new time object only based on the day, month and year (ie. the same day at 00:00).

__Example:__ Retrieve all the users whose birthday is today

```py
r.table("users").filter(lambda user:
    user["birthdate"].date() == r.now().date()
).run(conn)
```


