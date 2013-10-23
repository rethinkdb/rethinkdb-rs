---
layout: api-command 
language: JavaScript
permalink: api/javascript/time_of_day/
command: timeOfDay 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/dates-and-times/timeOfDay.md
io:
    -   - time
        - number
related_commands:
    now: now/
    time: time/
    inTimezone: in_timezone/
---

{% apibody %}
time.timeOfDay() &rarr; number
{% endapibody %}

Return the number of seconds elapsed since the beginning of the day stored in the time object.

__Example:__ Retrieve posts that were submitted before noon.

```js
r.table("posts").filter(
    r.row("date").timeOfDay().le(12*60*60)
).run(conn, callback)
```



