---
layout: api-command
language: JavaScript
permalink: api/javascript/timezone/
command: timezone
io:
    -   - time
        - time
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

```javascript
r.table("users").filter( function(user) {
    return user("subscriptionDate").timezone().eq("-07:00")
})
```


