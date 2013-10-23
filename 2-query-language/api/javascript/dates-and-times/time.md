---
layout: api-command 
language: JavaScript
permalink: api/javascript/time/
command: time 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/dates-and-times/time.md
io:
    -   - r
        - time
related_commands:
    now: now/
    epochTime: epoch_time/
    ISO8601: iso8601/
---

# Command syntax #

{% apibody %}
r.time(year, month, day[, hour, minute, second], timezone)
    &rarr; time
{% endapibody %}

# Description #

Create a time object for a specific time.

__Example:__ Update the birthdate of the user "John" to November 3rd, 1986 UTC.

```js
r.table("user").get("John").update({birthdate: r.time(1986, 11, 3, 'Z')}).run(conn, callback)
```


