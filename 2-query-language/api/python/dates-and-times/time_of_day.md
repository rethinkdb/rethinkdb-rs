---
layout: api-command 
language: Python
permalink: api/python/time_of_day/
command: time_of_day 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/dates-and-times/time_of_day.md
related_commands:
    now: now/
    time: time/
    in_timezone: in_timezone/
---

{% apibody %}
time.time_of_day() &rarr; number
{% endapibody %}

Return the number of seconds elapsed since the beginning of the day stored in the time object.

__Example:__ Retrieve posts that were submitted before noon.

```py
r.table("posts").filter(
    r.row["date"].time_of_day() <= 12*60*60
).run(conn)
```


