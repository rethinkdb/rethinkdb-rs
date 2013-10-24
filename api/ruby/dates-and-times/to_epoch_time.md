---
layout: api-command 
language: Ruby
permalink: api/ruby/to_epoch_time/
command: to_epoch_time 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/dates-and-times/to_epoch_time.md
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.to_epoch_time() &rarr; number
{% endapibody %}

# Description #

Convert a time object to its epoch time.

__Example:__ Return the current time in an ISO8601 format.

```rb
r.now().to_epoch_time()
```
