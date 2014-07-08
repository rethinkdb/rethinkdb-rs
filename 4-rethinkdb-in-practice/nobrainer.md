---
layout: documentation
title: RethinkDB and Rails using NoBrainer
active: docs
docs_active: nobrainer
permalink: docs/nobrainer/
---

# Getting Started

[NoBrainer](http://nobrainer.io) is a RethinkDB ORM, which is an
almost drop-in replacement for ActiveRecord in Ruby on Rails. This is
a guide to quickly getting up and running with NoBrainer. It assumes
some familiarity with Ruby on Rails and ActiveRecord. In particular,
it'll point out some of the differences between NoBrainer and
ActiveRecord.

{% infobox %}
**Before you start**

* Read the [thirty-second quickstart](/docs/quickstart)
* Ensure you have [RethinkDB installed](/docs/install) for your platform.
* Install [Ruby on Rails](http://rubyonrails.org/download/)

{% endinfobox %}

First, generate a new rails application using the NoBrainer template
like this:

```bash
$ rails new nb_app -O -m \
 https://raw.githubusercontent.com/deontologician/nobrainer/master/template.rb
$ cd nb_app
```

The `-O` option prevents rails from generating boilerplate for
ActiveRecord. We aren't going to need that since we're using NoBrainer
instead.

NoBrainer has a model generator, which you can use individually or
with the scaffolding mechanism. For example, here's a scaffold for an
Article resource:

```bash
$ rails g scaffold Article title:String text:String tags:Array
```

This yields the following model in `app/models/article.rb`:

```ruby
class Article
  include NoBrainer::Document

  field :title, :type => String
  field :author, :type => String
  field :tags, :type => Array

end
```

Unlike in a relational database, RethinkDB doesn't enforce types, so
NoBrainer's type annotations on the field are validators that are run
just before a document is saved to the database. If you don't want to
specify the type for a field, you can just omit it in the generator
invocation:

```bash
$ rails g model User name:String:index user_data
```

This allows the `user_data` field to contain any legal JSON value,
while `name` must still be a valid string.

```ruby
class User
  include NoBrainer::Document

  field :name, :type => String
  field :custom_data

  index :name
end
```

You'll notice this also created a simple secondary index on the `name`
field down at the bottom of the
class. [NoBrainer can handle different index types](http://nobrainer.io/docs/indexes/)
as well. In order to create the index in the database, use the Rake
task:

```bash
$ rake db:update_indexes
```

You can also specify associations between models in the generator:

```bash
$ rails g model Comment body:String liked:Boolean \
    user:belongs_to article:belongs_to
```

This will create the following model for comments:

```ruby
class Comment
  include NoBrainer::Document

  field :body, :type => String
  field :liked, :type => Boolean
  belongs_to :user
  belongs_to :article

end
```

If we go back into the `Article` model and add the `has_many` side of
the association, it's important to note that `has_many` associations
in NoBrainer are read-only. NoBrainer leaves saving the members of the
association to the developer because the server cannot enforce strong
consistency due to the lack of transactions.

NoBrainer also supports more in-depth validations on the fields. We
can modify the `Article` model in a few ways to ensure the data has
the properties we expect:

```ruby
class Article
  include NoBrainer::Document

  has_many :comments # read only!

  field :title, :type => String
  field :text,  :type => String, :required => true
  field :tags,  :type => Array, :default => []

  validates :title, :length => { minimum: 5 }
end
```

NoBrainer runs the validations only when saving, but not when
retrieving a document. This means you can always retrieve your data,
but new bad data won't be inserted.

You can read more about what
[arguments the field method accepts](http://nobrainer.io/docs/fields/),
and you might want to read up on
[how validation works](http://nobrainer.io/docs/validations/).

# Nested resources in views

Since NoBrainer's `has_many` associations are read-only, handling
nested resources changes a little bit. For example, if `Comments` are
nested in `Articles`, the comment form in the Article view would look
like this:

```erb
<%= form_for([@article, Comment.new(:article => @article)]) do |f| %>
  <b>Make a comment:</b><br>
  <%= f.label "I liked this article" %> <%= f.check_box :liked %><br>
  <%= f.text_area :body %>
  <%= f.submit %>
<% end %>
```

This just creates a new `Comment` and associates it with the current
`Article`, rather than using the `build` method.

# Querying

NoBrainer adds a light wrapper around ReQL queries. Here are some examples:

```ruby
# Find a specific document by its primary key
Article.find "091234f8ad9e0123f"

# Find a comment from a user with 'bob' in its name sorted by the name.
# Note: NoBrainer will use the :name index from User by default
User.where(:name => /bob/).order_by(:name => :desc).to_a

# Get two random comments that did not like the article
Comment.where(:liked => false).sample(2)
```

There is very comprehensive documentation of
[NoBrainer's query language](http://nobrainer.io/docs/querying/), and
what capabilities it has. NoBrainer's language is built on top of
ReQL, so if you know how to do something in ReQL, it should be
straightforward to translate it.

# And that's it!

You're on your way with Ruby on Rails and RethinkDB! Be sure to check
out the [ReQL api docs](/api/ruby) if you're wondering what
capabilities RethinkDB has. If you'd like to see a a different style
of Ruby web app using RethinkDB for comparison, we have an
[annotated Sinatra example application](/docs/examples/sinatra-pastie/).

Finally, once you get past example applications and tutorials and
start building your own app, you may find yourself asking "How do
I...". Our
[cookbook section](/docs/cookbook/ruby/) is a
great resource for finding common queries and example snippets.

