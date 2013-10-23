---
layout: api-command 
language: Python
permalink: api/python/year/
command: year 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/dates-and-times/year.md
related_commands:
    now: now/
    time: time/
---

{% apibody %}
time.year() &rarr; number
{% endapibody %}

Return the year of a time object.

__Example:__ Retrieve all the users born in 1986.

```py
r.table("users").filter(lambda user:
    user["birthdate"].year() == 1986
).run(conn)
```


