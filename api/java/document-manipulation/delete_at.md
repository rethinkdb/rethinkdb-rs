---
layout: api-command
language: Java
permalink: api/java/delete_at/
command: deleteAt
related_commands:
    insertAt: insert_at/
    spliceAt: splice_at/
    changeAt: change_at/
---

# Command syntax #

{% apibody %}
array.deleteAt(index [,endIndex]) &rarr; array
{% endapibody %}

# Description #

Remove one or more elements from an array at a given index. Returns the modified array. (Note: `deleteAt` operates on arrays, not documents; to delete documents, see the [delete](/api/java/delete) command.)

If only `index` is specified, `deleteAt` removes the element at that index. If both `index` and `endIndex` are specified, `deleteAt` removes the range of elements between `index` and `endIndex`, inclusive of `index` but not inclusive of `endIndex`.

If `endIndex` is specified, it must not be less than `index`. Both `index` and `endIndex` must be within the array's bounds (i.e., if the array has 10 elements, an `index` or `endIndex` of 10 or higher is invalid).

By using a negative `index` you can delete from the end of the array. `-1` is the last element in the array, `-2` is the second-to-last element, and so on. You may specify a negative `endIndex`, although just as with a positive value, this will not be inclusive. The range `(2,-1)` specifies the third element through the next-to-last element.

__Example:__ Delete the second element of an array.

```js
> r(['a','b','c','d','e','f']).deleteAt(1).run(conn)
// result passed to callback
['a', 'c', 'd', 'e', 'f']
```

__Example:__ Delete the second and third elements of an array.

```js
> r(['a','b','c','d','e','f']).deleteAt(1,3).run(conn)
// result passed to callback
['a', 'd', 'e', 'f']
```

__Example:__ Delete the next-to-last element of an array.

```js
> r(['a','b','c','d','e','f']).deleteAt(-2).run(conn)
// result passed to callback
['a', 'b', 'c', 'd', 'f']
```

__Example:__ Delete a comment on a post.

Given a post document such as:

```js
{
    id: '4cf47834-b6f9-438f-9dec-74087e84eb63',
    title: 'Post title',
    author: 'Bob',
    comments: [
        { author: 'Agatha', text: 'Comment 1' },
        { author: 'Fred', text: 'Comment 2' }
    ]
}
```

The second comment can be deleted by using `update` and `deleteAt` together.

```js
r.table('posts').get('4cf47834-b6f9-438f-9dec-74087e84eb63').update({
    comments: r.row('comments').deleteAt(1)
}).run(conn)
```
