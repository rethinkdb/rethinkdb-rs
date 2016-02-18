---
layout: api-command
language: JavaScript
permalink: api/javascript/slice/
command: slice
io:
    -   - sequence
        - stream
    -   - array
        - array
    -   - binary
        - binary
related_commands:
    orderBy: order_by/
    skip: skip/
    limit: limit/
    nth: nth/
---

# Command syntax #

{% apibody %}
selection.slice(startOffset[, endOffset, {leftBound:'closed', rightBound:'open'}]) &rarr; selection
stream.slice(startOffset[, endOffset, {leftBound:'closed', rightBound:'open'}]) &rarr; stream
array.slice(startOffset[, endOffset, {leftBound:'closed', rightBound:'open'}]) &rarr; array
binary.slice(startOffset[, endOffset, {leftBound:'closed', rightBound:'open'}]) &rarr; binary
{% endapibody %}

# Description #

Return the elements of a sequence within the specified range.

`slice` returns the range between `startOffset` and `endOffset`. If only `startOffset` is specified, `slice` returns the range from that index to the end of the sequence. Specify `leftBound` or `rightBound` as `open` or `closed` to indicate whether to include that endpoint of the range by default: `closed` returns that endpoint, while `open` does not. By default, `leftBound` is closed and `rightBound` is open, so the range `(10,13)` will return the tenth, eleventh and twelfth elements in the sequence.

If `endOffset` is past the end of the sequence, all elements from `startOffset` to the end of the sequence will be returned. If `startOffset` is past the end of the sequence or `endOffset` is less than `startOffset`, a zero-element sequence will be returned.

Negative `startOffset` and `endOffset` values are allowed with arrays; in that case, the returned range counts back from the array's end. That is, the range `(-2)` returns the last two elements, and the range of `(2,-1)` returns the second element through the next-to-last element of the range. An error will be raised on a negative `startOffset` or `endOffset` with non-arrays. (An `endOffset` of &minus;1 *is* allowed with a stream if `rightBound` is closed; this behaves as if no `endOffset` was specified.)

If `slice` is used with a [binary](/api/javascript/binary) object, the indexes refer to byte positions within the object. That is, the range `(10,20)` will refer to the 10th byte through the 19th byte.

__Example:__ Return the fourth, fifth and sixth youngest players. (The youngest player is at index 0, so those are elements 3&ndash;5.)

```js
r.table('players').orderBy({index: 'age'}).slice(3,6).run(conn, callback)
```

__Example:__ Return all but the top three players who have a red flag.

```js
r.table('players').filter({flag: 'red'}).orderBy(r.desc('score')).slice(3).run(conn, callback)
```

__Example:__ Return holders of tickets `X` through `Y`, assuming tickets are numbered sequentially. We want to include ticket `Y`.

```js
r.table('users').orderBy('ticket').slice(x, y, {right_bound: 'closed'}).run(conn, callback)
```

__Example:__ Return the elements of an array from the second through two from the end (that is, not including the last two).

```js
r.expr([0,1,2,3,4,5]).slice(2,-2).run(conn, callback)
```

Result:

```js
[2,3]
```
