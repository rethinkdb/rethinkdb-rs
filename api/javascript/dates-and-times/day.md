---
layout: api-command 
language: JavaScript
permalink: api/javascript/day/
command: day 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/dates-and-times/day.md
io:
    -   - time
        - number
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.day() &rarr; number
{% endapibody %}

# Description #

Return the day of a time object as a number between 1 and 31.

__Example:__ Return the users born on the 24th of any month.

```js
r.table("users").filter(
    r.row("birthdate").day().eq(24)
).run(conn, callback)
```


