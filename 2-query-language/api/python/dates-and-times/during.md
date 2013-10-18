---
layout: api-command 
language: Python
permalink: api/python/during/
command: during
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/dates-and-times/during.md
---

{% apibody %}
time.during(start_time, end_time[, left_bound="open/closed", right_bound="open/closed"]) → bool
{% endapibody %}

Return if a time is between two other times (by default, inclusive for the start, exclusive for the end).

__Example:__ Retrieve all the posts that were posted between December 1st, 2013 (inclusive) and December 10th, 2013 (exclusive).

```py
r.table("posts").filter(
    r.row['date'].during(r.time(2013, 12, 1), r.time(2013, 12, 10))
).run(conn)
```


__Example:__ Retrieve all the posts that were posted between December 1st, 2013 (exclusive) and December 10th, 2013 (inclusive).

```py
r.table("posts").filter(
    r.row['date'].during(r.time(2013, 12, 1), r.time(2013, 12, 10), left_bound="open", right_bound="closed")
).run(conn)
```

