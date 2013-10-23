---
layout: api-command 
language: JavaScript
permalink: api/javascript/hours/
command: hours 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/dates-and-times/hours.md
io:
    -   - time
        - number
related_commands:
    now: now/
    time: time/
---

{% apibody %}
time.hours() → number
{% endapibody %}

Return the hour in a time object as a number between 0 and 23.

__Example:__ Return all the posts submitted after midnight and before 4am.

```js
r.table("posts").filter(function(post) {
    return post("date").hours().lt(4)
})
```

