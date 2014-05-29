---
layout: api-command
language: Ruby
permalink: api/ruby/slice/
command: 'slice, []'
related_commands:
    order_by: order_by/
    skip: skip/
    limit: limit/
    'nth, []': nth/
---

# Command syntax #

{% apibody %}
selection.slice(start_index[, end_index, :left_bound => 'closed', :right_bound =>'open']) &rarr; selection
stream.slice(start_index[, end_index, :left_bound => 'closed', :right_bound =>'open']) &rarr; stream
array.slice(start_index[, end_index, :left_bound => 'closed', :right_bound =>'open']) &rarr; array
{% endapibody %}

# Description #

Return the elements of a sequence within the specified range.

`slice` returns the range between `start_index` and `end_index`. If only `start_index` is specified, `slice` returns the range from that index to the end of the sequence. Specify `left_bound` or `right_bound` as `open` or `closed` to indicate whether to include that endpoint of the range by default: `closed` returns that endpoint, while `open` does not. By default, `left_bound` is closed and `right_bound` is open, so the range `(10,13)` will return the tenth, eleventh and twelfth elements in the sequence.

If `end_index` is past the end of the sequence, all elements from `start_index` to the end of the sequence will be returned. If `start_index` is past the end of the sequence or `end_index` is less than `start_index`, a zero-element sequence will be returned (although see below for negative `end_index` values). An error will be raised on a negative `start_index`.

A negative `end_index` is allowed with arrays; in that case, the returned range counts backward from the array's end. That is, the range of `(2,-1)` returns the second element through the next-to-last element of the range. A negative `end_index` is not allowed with a stream. (An `end_index` of &minus;1 *is* allowed with a stream if `right_bound` is closed; this behaves as if no `end_index` was specified.)

If you are only specifying the indexes and not the bounding options, you may use Ruby's range operator as a shorthand: `[start_index..end_index]`. Note that when you use this shorthand `right_bound` will be `closed` and thus include `end_index`.

**Example:** Return the fourth, fifth and sixth youngest players. (The youngest player is at index 0, so those are elements 3&ndash;5.)

```rb
r.table('players').order_by(:index => 'age').slice(3,6).run(conn)
```

Or, using Ruby's range operator:

```rb
r.table('players').filter({'class': 'amateur'})[10..19].run(conn)
```

**Example:** Return all but the top three players who have a red flag.

```rb
r.table('players').filter(:flag => 'red'}).order_by(:index => r.desc('score')).slice(3).run(conn)
```

**Example:** Return holders of tickets `X` through `Y`, assuming tickets are numbered sequentially. We want to include ticket `Y`.

```rb
r.table('users').order_by(:index => 'ticket').slice(x, y, :right_bound => 'closed').run(conn)
```

**Example:** Return the elements of an array from the second through two from the end (that is, not including the last two).

```rb
r.expr([0,1,2,3,4,5]).slice(2,-2).run(conn)
```
