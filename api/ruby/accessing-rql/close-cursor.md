---
layout: api-command
language: Ruby
permalink: api/ruby/close-cursor/
command: close
---

# Command syntax #

{% apibody %}
cursor.close
{% endapibody %}

# Description #


Close a cursor or a feed. Closing a cursor cancels the corresponding query and frees the memory
associated with the open request.

__Example:__ Close a cursor.

```rb
cursor.close
```
