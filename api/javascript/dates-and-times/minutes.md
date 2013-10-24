---
layout: api-command 
language: JavaScript
permalink: api/javascript/minutes/
command: minutes 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/dates-and-times/minutes.md
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

```js
r.table("posts").filter(function(post) {
    return post("date").minutes().lt(10)
})
```


