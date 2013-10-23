---
layout: api-command 
language: Python
permalink: api/python/day/
command: day
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/dates-and-times/day.md
related_commands:
    now: now/
    time: time/
---

{% apibody %}
time.day() &rarr; number
{% endapibody %}

Return the day of a time object as a number between 1 and 31.

__Example:__ Return the users born on the 24th of any month.

```py
r.table("users").filter(
    r.row["birthdate"].day() == 24
)
```


