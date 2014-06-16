---
layout: api-command
language: JavaScript
permalink: api/javascript/delete_at/
command: deleteAt
io:
    -   - array
        - array
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

Remove one or more elements from an array at a given index. Returns the modified array.

If only `index` is specified, `deleteAt` removes the element at that index. If both `index` and `endIndex` are specified, `deleteAt` removes the range of elements between `index` and `endIndex`, inclusive of `index` but not inclusive of `endIndex`.

__Example:__ Delete the second element of an array.

```js
> r(['a','b','c','d','e','f']).deleteAt(1).run(conn, callback)

['a', 'c', 'd', 'e', 'f']
```

__Example:__ Delete the second and third elements of an array.

```js
> r(['a','b','c','d','e','f']).deleteAt(1,3).run(conn, callback)

['a', 'd', 'e', 'f']
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
}).run(conn, callback)
```
