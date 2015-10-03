---
layout: api-command
language: Java
permalink: api/javascript/date/
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

__Example:__ Retrieve all the users whose birthday is today

```js
r.table("users").filter(function(user) {
    return user("birthdate").date().eq(r.now().date())
}).run(conn)
```


