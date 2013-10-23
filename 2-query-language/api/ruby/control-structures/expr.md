---
layout: api-command 
language: Ruby
permalink: api/ruby/expr/
command: expr 
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/ruby/control-structures/expr.md
---

# Command syntax #

{% apibody %}
r.expr(value) &rarr; value
{% endapibody %}

# Description #

Construct a RQL JSON object from a native object.

__Example:__ Objects wrapped with expr can then be manipulated by RQL API functions.

```rb
r.expr({:a => 'b'}).merge({:b => [1,2,3]}).run(conn)
```

__Example:__ In Ruby, you can also do this with just r.

```rb
r.expr({:a => 'b'}).merge({:b => [1,2,3]}).run(conn)
```

