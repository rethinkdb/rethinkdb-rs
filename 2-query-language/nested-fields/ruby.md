---
layout: documentation
title: Accessing nested fields
active: docs
docs_active: nested-fields
permalink: docs/nested-fields/ruby/
alias: docs/nested-fields/
switcher: true
language: Ruby
---

A ReQL document is a JSON object: a set of key-value pairs, in which each value might be a single value, a list of values, or *another* set of key-value pairs. When the value of a field contains more fields, we describe these as *nested fields.*

Consider a user table with contact information and a list of notes for each user in this format:

```rb
{
  :id => 10001,
  :name => "Bob Smith",
  :contact => {
    :phone => {
      :work => "408-555-1212",
      :home => "408-555-1213",
      :cell => "408-555-1214"
    },
    :email => {
      :work => "bob@smith.com",
      :home => "bobsmith@gmail.com",
      :other => "bobbys@moosecall.net"
    },
    :im => {
      :skype => "Bob Smith",
      :aim => "bobmoose",
      :icq => "nobodyremembersicqnumbers"
    }
  },
  :notes => [
    {
      :date => r.time(2014,1,1,'Z'),
      :from => "John Doe",
      :subject => "My name is even more boring than Bob's"
    },
    {
      :date => r.time(2014,2,2,'Z'),
      :from => "Bob Smith Sr",
      :subject => "Happy Second of February"
    }
  ]
}
```

The contact information is *nested,* like paths in a file system.

> contact &rarr; phone &rarr; work &rarr; 408-555-1212

You can get the value of a specific field by using the `[]` operator successively to "drill down" in the document nesting:

```rb
> r.table('users').get(10001)['contact']['phone']['work'].run(conn)

'408-555-1212'
```

With most commands that take a field name string or the `[]` syntax above, you can also use a JSON-style nested syntax:

```rb
> r.table('users').get(10001).pluck(
  {'contact' => {'phone' => 'work'}}
).run(conn)

{
  'contact' => {
    'phone' => {
      'work' => '408-555-1212'
    }
  }
}
```

In that example, when you're trying to get at just one value, the JSON style doesn't offer much advantage. But you can use it to retrieve *multiple* values at the same nesting level. For instance, you can get just Bob's work and cell numbers, but not home:

```rb
> r.table('users').get(10001).pluck(
  {'contact' => {'phone' => ['work', 'cell']}}
).run(conn)

{
  'contact' => {
    'phone' => {
      'cell' => '408-555-1214',
      'work' => '408-555-1212'
    }
  }
}
```

Or, Bob's work phone and Skype handle:

```rb
> r.table('users').get(10001).pluck(
  {'contact' => [{'phone' => 'work', 'im' => 'skype'}]}
).run(conn)

{
  'contact' => {
    'im' => {
      'skype' => 'Bob Smith'
    },
    'phone' => {
      'work' => '408-555-1212'
    }
  }
}
```

And there's more! You can filter on fields of objects inside a list. Suppose you wanted just the dates and senders of notes to Bob:

```rb
> r.table('users').get(10001).pluck(
  {'notes' => ['date', 'from']}
).run(conn)

{
  'notes' => [
    {
      'date' => 2014-01-01 00:00:00 +0000 ,
      'from' =>  'John Doe'
    },
    {
      'date' => 2014-02-02 00:00:00 +0000 ,
      'from' =>  'Bob Smith Sr.'
    }
  ]
}
```

If you ask for a nested field that doesn't exist, you will get an empty object or array (this is *not* the same as a `nil` value):

```rb
> r.table('users').get(10001).pluck(
  {'contact' => [{'phone' => 'work', 'im' => 'msn'}]}
).run(conn)

{
  'contact' => {
    'im' => { },
    'phone' => {
      'work' => '408-555-1212'
    }
  }
}
```

Be aware this behavior holds true when retrieving data from lists, too. If you extracted `subject` from `notes` above and Bob had 10 notes, 7 of which contained no `subject` field,  you would still get a list of 10 objects: 7 of them would be `{subject => <text>}` and 3 of them would be empty, i.e., `{ }`.

Also, another caveat: the nested field syntax doesn't guarantee identical schemas between documents that it returns. It's possible to describe a path that matches objects that have different schema, as seen in this simple example.

```rb
> r.expr([
  {
    'a' => {
      'b' => 1,
      'c' => 2
    }
  },
  {
    'a' => [
      {
        'b' => 1,
        'c' => 2
      }
    ]
  }
]).pluck({'a' => {'b' => true}}).run(conn)

[
  {
    'a' => {
      'b' => 1
    }
  },
  {
    'a' => [
      {
        'b' => 1
      }
    ]
  }
]
```
