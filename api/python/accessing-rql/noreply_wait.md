---
layout: api-command 
language: Python
permalink: api/python/noreply_wait/
command: noreply_wait
related_commands:
    run: run/
    sync: sync/
---

# Command syntax #

{% apibody %}
conn.noreply_wait()
{% endapibody %}

# Description #

`noreply_wait` ensures that previous queries with the `noreply` flag have been processed
by the server. Note that this guarantee only applies to queries run on the given connection.

__Example:__ We have previously run queries with the `noreply` argument set to `True`. Now
wait until the server has processed them.

```py
conn.noreply_wait()
```

