---
layout: api-command
language: Ruby
permalink: api/ruby/distinct/
command: distinct
related_commands:
    map: map/
    concat_map: concat_map/
    group: group/
---


# Command syntax #

{% apibody %}
sequence.distinct() &rarr; array
table.distinct() &rarr; stream
table.distinct(:index => <indexname>) &rarr; stream
{% endapibody %}

# Description #

Removes duplicate elements from a sequence.

The `distinct` command can be called on any sequence, a table, or called on a table with an index.

__Example:__ Which unique villains have been vanquished by marvel heroes?

```rb
r.table('marvel').concat_map{|hero| hero[:villain_list]}.distinct.run(conn)
```

__Example:__ Topics in a table of messages have a secondary index on them, and more than one message can have the same topic. What are the unique topics in the table?

```rb
r.table('messages').distinct({:index => 'topics'}).run(conn)
```

The above structure is functionally identical to:

```rb
r.table('messages')['topics'].distinct().run(conn)
```

However, the first form (passing the index as an argument to `distinct`) is faster, and won't run into array limit issues since it's returning a stream.
