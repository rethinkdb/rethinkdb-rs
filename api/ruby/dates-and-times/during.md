---
layout: api-command 
language: Ruby
permalink: api/ruby/during/
command: during
related_commands:
    now: now/
    time: time/
    in_timezone: in_timezone/
---

# Command syntax #

{% apibody %}
time.during(start_time, end_time[, left_bound="open/closed", right_bound="open/closed"])
    &rarr; bool
{% endapibody %}

# Description #

Return if a time is between two other times (by default, inclusive for the start, exclusive for the end).

__Example:__ Retrieve all the posts that were posted between December 1st, 2013 (inclusive) and December 10th, 2013 (exclusive).

```rb
r.table("posts").filter{ |post|
    post['date'].during(r.time(2013, 12, 1, "Z"), r.time(2013, 12, 10, "Z"))
}.run(conn)
```


__Example:__ Retrieve all the posts that were posted between December 1st, 2013
(exclusive) and December 10th, 2013 (inclusive).

```rb
r.table("posts").filter{ |post|
    post['date'].during(r.time(2013, 12, 1, "Z"), r.time(2013, 12, 10, "Z"),
        :left_bound => "open",
        :right_bound => "closed")
}.run(conn)
```

