---
layout: api-command 
language: Ruby
permalink: api/ruby/is_empty/
command: is_empty 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/transformations/is_empty.md
related_commands:
    indexes_of: indexes_of/
---

{% apibody %}
sequence.is_empty() → bool
{% endapibody %}

Test if a sequence is empty.

__Example:__ Are there any documents in the marvel table?

```rb
r.table('marvel').is_empty().run(conn)
```


