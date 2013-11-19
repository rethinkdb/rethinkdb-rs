---
layout: api-command 
language: JavaScript
permalink: api/javascript/epoch_time/
command: epochTime
io:
    -   - r
        - time
related_commands:
    now: now/
    time: time/
    ISO8601: iso8601/
---

# Command syntax #

{% apibody %}
r.epochTime(epochTime) &rarr; time
{% endapibody %}

# Description #

Create a time object based on seconds since epoch. The first argument is a float and
will be rounded to three decimal places (millisecond-precision).

__Example:__ Update the birthdate of the user "John" to November 3rd, 1986.

```js
r.table("user").get("John").update({birthdate: r.epochTime(531360000)}).run(conn, callback)
```
