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
    -	- binary
    	- binary
related_commands:
    order_by: order_by/
    skip: skip/
    limit: limit/
    nth: nth/
---

# Command syntax #

{% apibody %}
selection.slice(startIndex[, endIndex, {leftBound:'closed', rightBound:'open'}]) &rarr; selection
stream.slice(startIndex[, endIndex, {leftBound:'closed', rightBound:'open'}]) &rarr; stream
array.slice(startIndex[, endIndex, {leftBound:'closed', rightBound:'open'}]) &rarr; array
binary.slice(startIndex[, endIndex, {leftBound:'closed', rightBound:'open'}]) &rarr; binary
{% endapibody %}

# Description #

Return the elements of a sequence within the specified range.

`slice` returns the range between `startIndex` and `endIndex`. If only `startIndex` is specified, `slice` returns the range from that index to the end of the sequence. Specify `leftBound` or `rightBound` as `open` or `closed` to indicate whether to include that endpoint of the range by default: `closed` returns that endpoint, while `open` does not. By default, `leftBound` is closed and `rightBound` is open, so the range `(10,13)` will return the tenth, eleventh and twelfth elements in the sequence.

If `endIndex` is past the end of the sequence, all elements from `startIndex` to the end of the sequence will be returned. If `startIndex` is past the end of the sequence or `endIndex` is less than `startIndex`, a zero-element sequence will be returned (although see below for negative `endIndex` values). An error will be raised on a negative `startIndex`.

A negative `endIndex` is allowed with arrays; in that case, the returned range counts backward from the array's end. That is, the range of `(2,-1)` returns the second element through the next-to-last element of the range. A negative `endIndex` is not allowed with a stream. (An `endIndex` of &minus;1 *is* allowed with a stream if `rightBound` is closed; this behaves as if no `endIndex` was specified.)

If `slice` is used with a [binary](/api/javascript/binary) object, the indexes refer to byte positions within the object. That is, the range `(10,20)` will refer to the 10th byte through the 19th byte.

**Example:** Return the fourth, fifth and sixth youngest players. (The youngest player is at index 0, so those are elements 3&ndash;5.)

```js
r.table('players').orderBy({index: 'age'}).slice(3,6).run(conn, callback)
```

**Example:** Return all but the top three players who have a red flag.

```js
r.table('players').filter({flag: 'red'}).orderBy(r.desc('score')).slice(3).run(conn, callback)
```

**Example:** Return holders of tickets `X` through `Y`, assuming tickets are numbered sequentially. We want to include ticket `Y`.

```js
r.table('users').orderBy('ticket').slice(x, y, {right_bound: 'closed'}).run(conn, callback)
```

**Example:** Return the elements of an array from the second through two from the end (that is, not including the last two).

```js
r.expr([0,1,2,3,4,5]).slice(2,-2).run(conn, callback)
```

Result:

```js
[2,3]
```
