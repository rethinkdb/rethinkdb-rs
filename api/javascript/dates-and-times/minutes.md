---
layout: api-command
language: JavaScript
permalink: api/javascript/minutes/
command: minutes
io:
    -   - time
        - number
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.minutes() &rarr; number
{% endapibody %}

# Description #

Return the minute in a time object as a number between 0 and 59.

__Example:__ Return all the posts submitted during the first 10 minutes of every hour.

```javascript
r.table("posts").filter(function(post) {
    return post("date").minutes().lt(10)
})
```


