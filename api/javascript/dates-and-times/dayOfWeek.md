---
layout: api-command 
language: JavaScript
permalink: api/javascript/day_of_week/
command: dayOfWeek
io:
    -   - time
        - number
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.dayOfWeek() &rarr; number
{% endapibody %}

# Description #

Return the day of week of a time object as a number between 1 and 7 (following ISO 8601 standard). For your convenience, the terms r.monday, r.tuesday etc. are defined and map to the appropriate integer.

__Example:__ Return today's day of week.

```js
r.now().dayOfWeek().run(conn, callback)
```

__Example:__ Retrieve all the users who were born on a Tuesday.

```js
r.table("users").filter(
    r.row("birthdate").dayOfWeek().eq(r.tuesday)
)
```

