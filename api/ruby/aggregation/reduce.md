---
layout: api-command
language: Ruby
permalink: api/ruby/reduce/
command: reduce
related_commands:
    group: group/
    map: map/
    concat_map: concat_map/
    sum: sum/
    avg: avg/
    min: min/
    max: max/
---

# Command syntax #

{% apibody %}
sequence.reduce(reduction_function) &rarr; value
{% endapibody %}

# Description #

Produces a single value from a sequence by repeatedly calling the
reduction function.  The reduction function should take two arguments
and combine them together.  The arguments to the reduction function
can be either elements of the stream, or the results of a previous
call to the reduction function.

__Example:__ What's the product of all the bonus multipliers in game 7324?

```rb
r.table('games').get(7324)['bonus_multipliers'].reduce{|a,b| a*b}.run(conn)
```

__Example:__ Return a string containing all usernames, one per line.

```rb
r.table('users').map{|user| user['name']}.reduce{|a,b| a + "\n" + b}.run(conn)
```

__INCORRECT Example:__ This doesn't work:

```rb
r.table('users').reduce{|a,b| a + "\n" + b['name']}.run(conn) # WRONG!
```

If you need to reduce over a particular field of a document, you
should first `map` over the sequence of documents to pull out that
field, and only then reduce over it.  Alternatively, you can do this:

```rb
r.table('users').reduce{|a,b| {name: a['name'] + "\n" + b['name']}}.run(conn)
```

In other words, `reduce` isn't `fold`.  It doesn't proceed
left-to-right, it proceeds in parallel.

__Example:__ Return the geometric mean of a sequence of numbers.  The
expensive part (the map/reduce) is done lazily on the shards, and the
cheap part is done in javascript on the node hosting the query.

```rb
sequence.map {|n|
  {num: n, count: 1}
}.reduce {|a,b|
  {num: a['num'] * b['num'], count: a['count'] + b['count']}
}.do(r.js("(function(obj) {
  return Math.pow(obj.num, 1/obj.count);
})")).run(conn)
```
