---
layout: api-command
language: JavaScript
permalink: api/javascript/date/
command: date
io:
    -   - time
        - time
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

```javascript
r.table("users").filter(function(user) {
    return user("birthdate").date().eq(r.now().date())
}).run(conn, callback)
```

<!-- stop -->

Note that the [now][] command always returns UTC time, so the comparison may fail if `user("birthdate")` isn't also in UTC. You can use the [inTimezone][itz] command to adjust for this:

```javascript
r.table("users").filter(function(user) {
    return user("birthdate").date().eq(r.now().inTimezone("-08:00").date())
}).run(conn, callback)
```

[now]: /api/javascript/now/
[itz]: /api/javascript/in_timezone/
