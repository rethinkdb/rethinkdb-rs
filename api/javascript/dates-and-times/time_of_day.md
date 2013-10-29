---
layout: api-command 
language: JavaScript
permalink: api/javascript/time_of_day/
command: timeOfDay 
io:
    -   - time
        - number
related_commands:
    now: now/
    time: time/
    inTimezone: in_timezone/
---

# Command syntax #

{% apibody %}
time.timeOfDay() &rarr; number
{% endapibody %}

# Description #

Return the number of seconds elapsed since the beginning of the day stored in the time object.

__Example:__ Retrieve posts that were submitted before noon.

```js
r.table("posts").filter(
    r.row("date").timeOfDay().le(12*60*60)
).run(conn, callback)
```



