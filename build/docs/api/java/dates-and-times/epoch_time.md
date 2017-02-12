---
layout: api-command
language: Java
permalink: api/java/epoch_time/
command: epochTime
related_commands:
    now: now/
    time: time/
    ISO8601: iso8601/
---

# Command syntax #

{% apibody %}
r.epochTime(number) &rarr; time
{% endapibody %}

# Description #

Create a time object based on seconds since epoch. The first argument is a double and
will be rounded to three decimal places (millisecond-precision).

__Example:__ Update the birthdate of the user "John" to November 3rd, 1986.

```java
r.table("user").get("John").update(
    r.hashMap(birthdate, r.epochTime(531360000))
).run(conn);
```
