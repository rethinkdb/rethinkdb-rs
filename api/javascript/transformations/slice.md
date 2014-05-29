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
related_commands:
	order_by: order_by/
    skip: skip/
    limit: limit/
    nth: nth/
---

# Command syntax #

{% apibody %}
sequence.slice(startIndex[, endIndex, {leftBound:'closed', rightBound:'open'}]) &rarr; selection
stream.slice(startIndex[, endIndex, {leftBound:'closed', rightBound:'open'}]) &rarr; stream
array.slice(startIndex[, endIndex, {leftBound:'closed', rightBound:'open'}]) &rarr; array
{% endapibody %}

# Description #

Return the elements within a sequence within the specified range.

`slice` returns the range between `startIndex` and `endIndex`. If only `startIndex` is specified, `slice` returns the range from that index to the end of the sequence. Specify `leftBound` or `rightBound` as `open` or `closed` to indicate whether to include that endpoint of the range by default: `closed` returns that endpoint, while `open` does not. By default, `leftBound` is closed and `rightBound` is open, so the range `(10,13)` will return the tenth, eleventh and twelfth elements in the sequence.

If `endIndex` is past the end of the sequence, all elements from `startIndex` to the end of the sequence will be returned. If `startIndex` is past the end of the sequence, a zero-element sequence will be returned. An error will be raised on a negative `startIndex` or `endIndex`. (An `endIndex` of &minus;1 *is* allowed if `rightBound` is closed; this behaves as if no `endIndex` was specified.)

**Example:** Return players 11-20 (index positions 10 through 19) in the amateur class.

```js
r.table('players').filter({class: 'amateur'}).slice(10, 20).run(conn, callback)
```

**Example:** Return all but the top three players who have a red flag.

```js
r.table('players').filter({flag: 'red'}).orderBy(r.desc('score')).slice(3).run(conn, callback)
```

**Example:** Return holders of tickets `X` through `Y`, assuming tickets are numbered sequentially. We want to include ticket `Y`.

```js
r.table('users').orderBy('ticket').slice(x, y, {right_bound: 'closed'}).run(conn, callback)
```
