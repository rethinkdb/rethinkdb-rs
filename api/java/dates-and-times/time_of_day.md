---
layout: api-command
language: Java
permalink: api/java/time_of_day/
command: timeOfDay
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

```java
r.table("posts").filter(
    r.row("date").timeOfDay().le(12*60*60)
).run(conn)
```



