---
layout: api-command 
language: Ruby
permalink: api/ruby/splice_at/
command: splice_at 
related_commands:
    insert_at: insert_at/
    delete_at: delete_at/
    change_at: change_at/
---

# Command syntax #

{% apibody %}
array.splice_at(index, array) &rarr; array
{% endapibody %}

# Description #

Insert several values in to an array at a given index. Returns the modified array.

__Example:__ Hulk and Thor decide to join the avengers.

```rb
r.expr(["Iron Man", "Spider-Man"]).splice_at(1, ["Hulk", "Thor"]).run(conn)
```


