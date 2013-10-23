---
layout: api-command 
language: Python
permalink: api/python/minutes/
command: minutes
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/dates-and-times/minutes.md
related_commands:
    now: now/
    time: time/
    in_timezone: in_timezone/
---
{% apibody %}
time.minutes() &rarr; number
{% endapibody %}

Return the minute in a time object as a number between 0 and 59.

__Example:__ Return all the posts submitted during the first 10 minutes of every hour.

```py
r.table("posts").filter(lambda post:
    post["date"].minutes() < 10
).run(conn)
```
