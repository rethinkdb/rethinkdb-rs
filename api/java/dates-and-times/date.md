---
layout: api-command
language: Java
permalink: api/java/date/
command: date
related_commands:
    now: now/
    time: time/
    inTimezone: in_timezone/
---

# Command syntax #

{% apibody %}
time.date() &rarr; time
{% endapibody %}

# Description #

Return a new time object only based on the day, month and year (ie. the same day at 00:00).

__Example:__ Retrieve all the users whose birthday is today.

```java
r.table("users").filter(
    user -> user.g("birthdate").date().eq(r.now().date())
).run(conn);
```


