---
layout: api-command
language: Python
permalink: api/python/slice/
command: 'slice, []'
related_commands:
    order_by: order_by/
    skip: skip/
    limit: limit/
    'nth, []': nth/
---

# Command syntax #

{% apibody %}
selection.slice(start_index[, end_index, left_bound='closed', right_bound='open']) &rarr; selection
stream.slice(start_index[, end_index, left_bound='closed', right_bound='open']) &rarr; stream
array.slice(start_index[, end_index, left_bound='closed', right_bound='open']) &rarr; array
binary.slice(start_index[, end_index, left_bound='closed', right_bound='open']) &rarr; binary
{% endapibody %}

# Description #

Return the elements of a sequence within the specified range.

`slice` returns the range between `start_index` and `end_index`. If only `start_index` is specified, `slice` returns the range from that index to the end of the sequence. Specify `left_bound` or `right_bound` as `open` or `closed` to indicate whether to include that endpoint of the range by default: `closed` returns that endpoint, while `open` does not. By default, `left_bound` is closed and `right_bound` is open, so the range `(10,13)` will return the tenth, eleventh and twelfth elements in the sequence.

If `end_index` is past the end of the sequence, all elements from `start_index` to the end of the sequence will be returned. If `start_index` is past the end of the sequence or `end_index` is less than `start_index`, a zero-element sequence will be returned.

Negative `start_index` and `end_index` values are allowed with arrays; in that case, the returned range counts back from the array's end. That is, the range `(-2)` returns the last two elements, and the range of `(2,-1)` returns the second element through the next-to-last element of the range. An error will be raised on a negative `start_index` or `end_index` with non-arrays. (An `end_index` of &minus;1 *is* allowed with a stream if `right_bound` is closed; this behaves as if no `end_index` was specified.)

If `slice` is used with a [binary](/api/python/binary) object, the indexes refer to byte positions within the object. That is, the range `(10,20)` will refer to the 10th byte through the 19th byte.

If you are only specifying the indexes and not the bounding options, you may use Python's slice operator as a shorthand: `[start_index:end_index]`.

__Example:__ Return the fourth, fifth and sixth youngest players. (The youngest player is at index 0, so those are elements 3&ndash;5.)

```py
r.table('players').order_by(index='age').slice(3,6).run(conn)
```

Or, using Python's slice operator:

```py
r.table('players').filter({'class': 'amateur'})[10:20].run(conn)
```

__Example:__ Return all but the top three players who have a red flag.

```py
r.table('players').filter({'flag': 'red'}).order_by(index=r.desc('score')).slice(3).run(conn)
```

__Example:__ Return holders of tickets `X` through `Y`, assuming tickets are numbered sequentially. We want to include ticket `Y`.

```py
r.table('users').order_by(index='ticket').slice(x, y, right_bound='closed').run(conn)
```

__Example:__ Return the elements of an array from the second through two from the end (that is, not including the last two).

```py
r.expr([0,1,2,3,4,5]).slice(2,-2).run(conn)
```

Result:

```py
[2,3]
```
