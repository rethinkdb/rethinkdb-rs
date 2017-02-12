---
layout: documentation
title: Using RethinkDB with Ruby on Rails
docs_active: rails
permalink: docs/rails/
---

It's easy to use RethinkDB with Ruby on Rails. This guide assumes some
familiarity with Rails and ActiveRecord. We'll be using
[NoBrainer](http://nobrainer.io)&mdash;a RethinkDB ORM, which is an
almost drop-in replacement for ActiveRecord.

{% toctag %}

{% infobox %}
**Before you start**

* Read the [thirty-second quickstart](/docs/quickstart)
* Ensure you have [RethinkDB installed](/docs/install) for your platform.
* Install [Ruby on Rails](http://rubyonrails.org/download/)

{% endinfobox %}

# Getting started

First, generate a new Rails application using NoBrainer:

```bash
$ rails new nb_app
$ cd nb_app
$ echo "gem 'nobrainer'" >> Gemfile
$ bundle install
$ rails g nobrainer:install
```

You can now generate models individually or use the scaffolding
mechanism. For example, here's a scaffold for an Article resource:

```bash
$ rails g scaffold Article title:string text:string tags:array
```

This yields the following model in `app/models/article.rb`:

```ruby
class Article
  include NoBrainer::Document
  include NoBrainer::Document::Timestamps

  field :title, :type => String
  field :text, :type => String
  field :tags, :type => Array
end
```

You're now up and running with RethinkDB and Rails!

# Models in depth

Unlike a relational database, RethinkDB doesn't enforce types, so
NoBrainer's type annotations on the field are validators that are run
just before a document is saved to the database. If you don't want to
specify the type for a field, you can use the dummy type `object`:

```bash
$ rails g model User name:string:index user_data:object
```

This allows the `user_data` field to contain any legal JSON value,
while `name` must still be a valid string.

```ruby
class User
  include NoBrainer::Document
  include NoBrainer::Document::Timestamps

  field :name, :type => String, :index => true
  field :custom_data
end
```

The NoBrainer generator automatically includes the
[TimeStamps](http://nobrainer.io/docs/timestamps) mixin that adds the
fields `created_on` and `updated_on`. You'll also notice this created
a simple secondary index on the `name`
field. In order to add the index to the database, you can use the
Rake task:

```bash
$ rake nobrainer:sync_schema
```

## Associations

You can specify associations between models in the generator:

```bash
$ rails g model Comment body:string liked:boolean \
    user:belongs_to article:belongs_to
```

This will create the following model for comments:

```ruby
class Comment
  include NoBrainer::Document
  include NoBrainer::Document::Timestamps

  field :body, :type => String
  field :liked, :type => Boolean
  belongs_to :user
  belongs_to :article
end
```

If we go back into the `Article` model and add the `has_many` side of
the association, it's important to note that `has_many` associations
in NoBrainer are read-only. The server doesn't support transactions,
so saving the members of the association is up to the developer.

## Validation

We can also specify more in-depth validation on fields. We can modify the
`Article` model in a few ways to ensure the data has the properties we
expect:

```ruby
class Article
  include NoBrainer::Document
  include NoBrainer::Document::Timestamps

  has_many :comments # read only!

  field :title, :type => String
  field :text,  :type => String, :required => true
  field :tags,  :type => Array, :default => []

  validates :title, :length => { minimum: 5 }
end
```

NoBrainer runs the validations only when saving, but not when
retrieving a document. This means you can always retrieve your data,
but an invalid model won't be saved to the database.

Read about [validation in
NoBrainer](http://nobrainer.io/docs/validations/) for more details.

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
Article.find "2FrYybOfzezVpT"

# Find a comment from a user with 'bob' in its name sorted by the name.
# Note: NoBrainer will use the :name index from User by default
User.where(:name => /bob/).order_by(:name => :desc).to_a

# Get two random comments that did not like the article
Comment.where(:liked => false).sample(2)
```

There is very comprehensive documentation of
[NoBrainer's query language](http://nobrainer.io/docs/querying/), and
its capabilities. NoBrainer's language is built on top of
ReQL, so if you know how to do something in ReQL, it should be
straightforward to translate it.

# And that's it!

You're on your way with Ruby on Rails and RethinkDB! Be sure to check
out additional resources for more information.

- The [NoBrainer documentation](http://nobrainer.io/).
- The [ReQL API docs](/api/ruby) for information on RethinkDB commands.
- The [Sinatra example
  application](https://github.com/rethinkdb/rethinkdb-example-sinatra-pastie) for a different style
  of a Ruby web app using RethinkDB.
- The [cookbook section](/docs/cookbook/ruby/) for common queries and
  example snippets.


