---
layout: api-command 
language: Python
permalink: api/python/epoch_time/
command: epoch_time 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/dates-and-times/epoch_time.md
related_commands:
    now: now/
    time: time/
    iso8601: iso8601/
---

{% apibody %}
r.epoch_time(epoch_time) &rarr; time
{% endapibody %}

Create a time object based on seconds since epoch.

__Example:__ Update the birthdate of the user "John" to November 3rd, 1986.

```py
r.table("user").get("John").update({"birthdate": r.epoch_time(531360000)}).run(conn)
```


