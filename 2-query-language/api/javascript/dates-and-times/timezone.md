---
layout: api-command 
language: JavaScript
permalink: api/javascript/timezone/
command: timezone 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/dates-and-times/timezone.md
io:
    -   - time
        - time
related_commands:
    inTimezone: in_timezone/
    now: now/
    time: time/
---

{% apibody %}
time.timezone() &rarr; string
{% endapibody %}

Return the timezone of the time object.

__Example:__ Return all the users in the "-07:00" timezone.

```js
r.table("users").filter( function(user) {
    return user("subscriptionDate").timezone().eq("-07:00")
})
```


