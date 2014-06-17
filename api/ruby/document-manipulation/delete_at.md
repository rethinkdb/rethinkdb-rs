---
layout: api-command
language: Ruby
permalink: api/ruby/delete_at/
command: delete_at
related_commands:
    insert_at: insert_at/
    splice_at: splice_at/
    change_at: change_at/
---


# Command syntax #

{% apibody %}
array.delete_at(index [,endIndex]) &rarr; array
{% endapibody %}

# Description #

Remove one or more elements from an array at a given index. Returns the modified array. (Note: `delete_at` operates on arrays, not documents; to delete documents, see the [delete](/api/ruby/delete) command.)

If only `index` is specified, `delete_at` removes the element at that index. If both `index` and `end_index` are specified, `delete_at` removes the range of elements between `index` and `end_index`, inclusive of `index` but not inclusive of `end_index`.

If `end_index` is specified, it must not be less than `index`. Both `index` and `end_index` must be within the array's bounds (i.e., if the array has 10 elements, an `index` or `end_index` of 10 or higher is invalid).

By using a negative `index` you can delete from the end of the array. `-1` is the last element in the array, `-2` is the second-to-last element, and so on. You may specify a negative `end_index`, although just as with a positive value, this will not be inclusive. The range `(2,-1)` specifies the third element through the next-to-last element.

__Example:__ Delete the second element of an array.

```rb
> r.expr(['a','b','c','d','e','f']).delete_at(1).run(conn)

['a', 'c', 'd', 'e', 'f']
```

__Example:__ Delete the second and third elements of an array.

```rb
> r.expr(['a','b','c','d','e','f']).delete_at(1,3).run(conn)

['a', 'd', 'e', 'f']
```

__Example:__ Delete the next-to-last element of an array.

```rb
> r.expr(['a','b','c','d','e','f']).delete_at(-2).run(conn)

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

The second comment can be deleted by using `update` and `delete_at` together.

```rb
r.table('posts').get('4cf47834-b6f9-438f-9dec-74087e84eb63').update{ |post|
    { :comments => post['comments'].delete_at(1) }
}.run(conn)
```
