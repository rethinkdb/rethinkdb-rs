---
layout: api-command 
language: JavaScript
permalink: api/javascript/seconds/
command: seconds 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/dates-and-times/seconds.md
io:
    -   - time
        - number
related_commands:
    now: now/
    time: time/
---

{% apibody %}
time.seconds() &rarr; number
{% endapibody %}

Return the seconds in a time object as a number between 0 and 59.999 (double precision).

__Example:__ Return the post submitted during the first 30 seconds of every minute.

```js
r.table("posts").filter(function(post) {
    return post("date").seconds().lt(30)
})
```

