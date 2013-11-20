---
layout: api-command 
language: Ruby
permalink: api/ruby/noreply_wait/
command: noreply_wait
related_commands:
    run: run/
    sync: sync/
---

# Command syntax #

{% apibody %}
conn.noreply_wait
{% endapibody %}

# Description #

Wait for outstanding no-reply requests on the given connection to finish. If you have
previously run queries with the `noreply` flag set, it guarantees that those queries
have been processed by the server.

__Example:__ We have previously run queries with the `noreply` argument set. Now
wait until the server has executed them.

```rb
conn.noreply_wait
```

