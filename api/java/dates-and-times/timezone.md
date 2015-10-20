---
layout: api-command
language: Java
permalink: api/java/timezone/
command: timezone
related_commands:
    inTimezone: in_timezone/
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.timezone() &rarr; string
{% endapibody %}

# Description #

Return the timezone of the time object.

__Example:__ Return all the users in the "-07:00" timezone.

```java
r.table("users").filter(
    user -> user.g("subscriptionDate").timezone().eq("-07:00")
).run(conn);
```


