---
layout: api-command 
language: Ruby
permalink: api/ruby/delete_at/
command: delete_at 
---


{% apibody %}
array.delete_at(index [,endIndex]) → array
{% endapibody %}

Remove an element from an array at a given index. Returns the modified array.

__Example:__ Hulk decides to leave the avengers.

```rb
r.expr(["Iron Man", "Hulk", "Spider-Man"]).delete_at(1).run(conn)
```

__Example:__ Hulk and Thor decide to leave the avengers.

```rb
r.expr(["Iron Man", "Hulk", "Thor", "Spider-Man"]).delete_at(1,3).run(conn)
```

