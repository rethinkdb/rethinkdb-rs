---
layout: api-command 
language: JavaScript
permalink: api/javascript/epoch_time/
command: epochTime
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/dates-and-times/epochTime.md
io:
    -   - r
        - time
related_commands:
    now: now/
    time: time/
    ISO8601: iso8601/
---

{% apibody %}
r.epochTime(epochTime) → time
{% endapibody %}

Create a time object based on seconds since epoch.

__Example:__ Update the birthdate of the user "John" to November 3rd, 1986.

```js
r.table("user").get("John").update({birthdate: r.epochTime(531360000)}).run(conn, callback)
```
