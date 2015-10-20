---
layout: api-command
language: Java
permalink: api/java/year/
command: year
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.year() &rarr; number
{% endapibody %}

# Description #

Return the year of a time object.

__Example:__ Retrieve all the users born in 1986.

```java
r.table("users").filter(
    user -> user.g("birthdate").year().eq(1986)
}).run(conn);
```
