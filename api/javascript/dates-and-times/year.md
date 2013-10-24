---
layout: api-command 
language: JavaScript
permalink: api/javascript/year/
command: year 
io:
    -   - time
        - number
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

```js
r.table("users").filter(function(user) {
    return user("birthdate").year().eq(1986)
}).run(conn, callback)
```
