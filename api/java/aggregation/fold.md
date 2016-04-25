---
layout: api-command
language: Java
permalink: api/java/fold/
command: fold
related_commands:
    reduce: reduce/
    concatMap: concat_map/
---

# Command syntax #

{% apibody %}
sequence.fold(base, function) &rarr; value
sequence.fold(base, function).optArg("emit", function)[.optArg("final_emit", function)] &rarr; sequence
{% endapibody %}

# Description #

Apply a function to a sequence in order, maintaining state via an accumulator. The `fold` command returns either a single value or a new sequence.

In its first form, `fold` operates like [reduce][rd], returning a value by applying a combining function to each element in a sequence. The combining function takes two parameters: the previous reduction result (the accumulator) and the current element. However, `fold` has the following differences from `reduce`:

* it is guaranteed to proceed through the sequence from first element to last.
* it passes an initial base value to the function with the first element in place of the previous reduction result.

{% apibody %}
combiningFunction(accumulator | base, element) &rarr; newAccumulator
{% endapibody %}

In its second form, `fold` operates like [concat_map][cm], returning a new sequence rather than a single value. When an `emit` function is provided, `fold` will:

* proceed through the sequence in order and take an initial base value, as above.
* for each element in the sequence, call both the combining function and a separate emitting function. The emitting function takes three parameters: the previous reduction result (the accumulator), the current element, and the output from the combining function (the new value of the accumulator).

If provided, the emitting function must return a list.

{% apibody %}
emit(previousAccumulator, element, accumulator) &rarr; array
{% endapibody %}

A `finalEmit` function may also be provided, which will be called at the end of the sequence. It takes a single parameter: the result of the last reduction through the iteration (the accumulator), or the original base value if the input sequence was empty. This function must return a list, which will be appended to `fold`'s output stream.

{% apibody %}
finalEmit(accumulator | base) &rarr; array
{% endapibody %}

[rd]: /api/java/reduce/
[cm]: /api/java/concat_map/

__Example:__ Concatenate words from a list.

```java
r.table("words").orderBy("id").fold("",
    (acc, word) -> acc.add(r.branch(r.eq(acc, ""), "", ", ")).add(word)
).run(conn);
```

(This example could be implemented with `reduce`, but `fold` will preserve the order when `words` is a RethinkDB table or other stream, which is not guaranteed with `reduce`.)

__Example:__ Return every other row in a table.

```java
r.table("even_things").fold(0,
    (acc, row) -> r.add(acc, 1)
).optArg("emit",
    (acc, row) -> r.branch(r.mod(acc, 2).eq(0), r.array(row), r.array())
).run(conn);
```

The first function increments the accumulator each time it's called, starting at `0`; the second function, the emitting function, alternates between returning a single-item list containing the current row or an empty list. The `fold` command will return a concatenated list of each emitted value.

__Example:__ Compute a five-day running average for a weight tracker.

```java
r.table("tracker").filter(
    r.hashMap("name", "bob")
).orderBy("date").g("weight").fold(r.array(),
    (acc, row) -> r.add(r.array(row), acc).limit(5)
).optArg("emit",
    (acc, row, new_acc) -> r.branch(new_acc.size().eq(5),
                                    r.array(new_acc.avg()),
                                    r.array())
).run(conn);
```
