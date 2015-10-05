---
layout: api-command
language: Java
permalink: api/java/minutes/
command: minutes
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

```java
r.table("posts").filter(function(post) {
    return post("date").minutes().lt(10)
})
```


