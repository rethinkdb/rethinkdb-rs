---
layout: api-command
language: JavaScript
permalink: api/javascript/http/
command: http
io:
    -   - r
        - value
---

# Command syntax #

{% apibody %}
r.http(url[, options]) &rarr; object
r.http(url[, options]) &rarr; string
{% endapibody %}

# Description #

Retrieve data from the specified URL using HTTP. Returns either a JSON object or a string depending on what the URL returns.

**Example:**

```js
var post = r.http('http://feedthing.com/feed.json', {allow_redirect: true}).run(conn, callback);
r.table('posts').insert(post).run(conn, callback);
```

# Options #

* `method`: HTTP method to use for the request. One of `GET`, `POST`, `PUT`, `PATCH`, `DELETE` or `HEAD`. Default: `GET`.

* `data`: Data to send to the server on a `POST`, `PUT`, `PATCH`, or `DELETE` request. For `PUT`, `PATCH` and `DELETE` requests, the value must be an object; it will be serialized to JSON and placed in the request body, and the `Content-Type` will be set to `application/json`.

	For `POST` requests, data may be either an object or a string. Objects will be written to the body as form-encoded key/value pairs (values must be numbers, strings, or `null`). Strings will be put directly into the body.

	If `data` is not a string or an object, an error will be thrown. If `data` is not specified, no data will be sent.

* `timeout`: Number of seconds to wait before timing out and aborting the operation. Default: 30.

* `reattempts`: Number of retries to make when connection errors occur. Default: 5.

* `allow_redirect`: Follow redirect and location headers, specified as a boolean. Default: false.

* `verify`: Verify the server's SSL certificate for HTTPS connections, specified as a boolean. Default: true.

* `params`: URL parameters to append to the URL as encoded key/value pairs, specified as an object. For example, `{query: 'banana', limit: 2}` will be appended as `?query=banana&limit=2`. Default: none.

* `header`: Extra header lines to include. The value may be an array of strings or an object. Default: none.

* `auth`: Authentication information in the form of an object with key/value pairs indicating the authentication type (in the `type` key) and any required information. Types currently supported are `basic` and `digest` for HTTP Basic and HTTP Digest authentication respectively. If `type` is omitted, `basic` is assumed. Example:

	```js
	r.http('http://feedthing.com/feed.json',
	    {auth: {type: 'basic', user: 'fred', password: 'mxyzptlk'}}
	).run(conn, callback)
	```

* `resultFormat`: The format the result should be returned in. The values can be `text` (always return as a string), `json` (parse the result as JSON, raising an error if the parsing fails), `jsonp` (parse the result as [padded JSON](http://www.json-p.org/)), or `auto` (determine the format based on the `Content-Type` returned from the URL). The default is `auto`.
