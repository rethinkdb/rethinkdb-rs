---
layout: documentation
title: Installing RethinkDB client drivers
docs_active: install-drivers
permalink: docs/install-drivers/
alias: docs/guides/drivers/
---

<section class="supported-platforms">
    <section class="platform-category">
        <h2>Official drivers</h2>
        <ul class="platform-buttons">
            <li>
                <a href="javascript/">
                    <img src="/assets/images/docs/driver-languages/javascript.png" />
                    <p class="name">JavaScript</p>
                </a>
            </li>
            <li>
                <a href="ruby/">
                    <img src="/assets/images/docs/driver-languages/ruby.png" />
                    <p class="name">Ruby</p>
                </a>
            </li>
            <li>
                <a href="python/">
                    <img src="/assets/images/docs/driver-languages/python.png" />
                    <p class="name">Python</p>
                </a>
            </li>
        </ul>
    </section>
    
    <section class="platform-category">
        <h2>Community-supported drivers</h2>
        <p>These drivers have been updated to use the JSON driver protocol and RethinkDB 1.13 ReQL (geometry support and basic changefeeds).</p>
        <ul class="platform-buttons">
            <li>
                <a href="https://github.com/mfenniak/rethinkdb-net">
                    <img src="/assets/images/docs/driver-languages/csharp.png" />
                    <p class="name">C# / .NET</p>
                </a>
            </li>
            <li>
                <a href="https://github.com/apa512/clj-rethinkdb">
                    <img src="/assets/images/docs/driver-languages/clojure.png" />
                    <p class="name smaller-font">Clojure <span class="driver-author">apa512</span></p>
                </a>
            </li>
            <li>
                <a href="https://github.com/orthecreedence/cl-rethinkdb">
                    <img src="/assets/images/docs/driver-languages/commonlisp.png" />
                    <p class="name smaller-font">Common Lisp</p>
                </a>
            </li>
            <li>
                <a href="https://github.com/dancannon/gorethink">
                    <img src="/assets/images/docs/driver-languages/go.png" />
                    <p class="name">Go</p>
                </a>
            </li>
            <li>
                <a href="http://hackage.haskell.org/package/rethinkdb">
                    <img src="/assets/images/docs/driver-languages/haskell.png" />
                    <p class="name">Haskell</p>
                </a>
            </li>
            <li>
                <a href="https://github.com/grandquista/Lua-ReQL">
                    <img src="/assets/images/docs/driver-languages/lua.png" />
                    <p class="name">Lua</p>
                </a>
            </li>
            <li>
                <a href="https://github.com/neumino/rethinkdbdash">
                    <img src="/assets/images/docs/driver-languages/nodejs.png" />
                    <p class="name">Node.js</p>
                </a>
            </li>
            <li>
                <a href="https://github.com/njlg/perl-rethinkdb">
                    <img src="/assets/images/docs/driver-languages/perl.png" />
                    <p class="name">Perl</p>
                </a>
            </li>
            <li>
                <a href="http://danielmewes.github.io/php-rql/">
                    <img src="/assets/images/docs/driver-languages/php.png" />
                    <p class="name">PHP</p>
                </a>
            </li>
            <li>
                <a href="https://github.com/kclay/rethink-scala">
                    <img src="/assets/images/docs/driver-languages/scala.png" />
                    <p class="name">Scala <span class="driver-author">kclay</span></p>
                </a>
            </li>
        </ul>
    </section>

    <section class="platform-category">
        <h2>Other community-supported drivers</h2>
        <p>These drivers have not been updated with more recent additions to the RethinkDB driver protocol, and will require work to take advantage of newer features. If you want to work with one of these languages, these may provide a good starting point.</p>
        <ul class="platform-buttons">
            <li>
            <a href="https://github.com/unbit/librethinkdb">
                <img src="/assets/images/docs/driver-languages/c.png" />
                <p class="name">C</p>
            </a>
            </li>
            <li>
            <a href="https://github.com/jurajmasar/rethink-db-cpp-driver">
                <img src="/assets/images/docs/driver-languages/cpp.png" />
                <p class="name">C++</p>
            </a>
            </li>
            <li>
                <a href="https://github.com/bitemyapp/revise">
                    <img src="/assets/images/docs/driver-languages/clojure.png" />
                    <p class="name smaller-font">Clojure <span class="driver-author">cesarbp</span></p>
                </a>
            </li>
            <li>
                <a href="https://github.com/dbettin/rethinkdb">
                    <img src="/assets/images/docs/driver-languages/dart.png" />
                    <p class="name smaller-font">Dart</p>
                </a>
            </li>
            <li>
            <a href="https://github.com/azukiapp/elixir-rethinkdb">
                <img src="/assets/images/docs/driver-languages/elixir.png" />
                <p class="name">Elixir</p>
            </a>
            </li>
            <li>
            <a href="https://github.com/taybin/lethink">
                <img src="/assets/images/docs/driver-languages/erlang.png" />
                <p class="name">Erlang</p>
            </a>
            </li>
            <li>
                <a href="https://github.com/npiv/rethink-java-driver">
                    <img src="/assets/images/docs/driver-languages/java.png" />
                    <p class="name">Java <span class="driver-author">npiv</span></p>
                </a>
            </li>
            <li>
                <a href="https://github.com/dkhenry/rethinkjava">
                    <img src="/assets/images/docs/driver-languages/java.png" />
                    <p class="name">Java <span class="driver-author">dkhenry</span></p>
                </a>
            </li>
            <li>
            <a href="https://github.com/dparnell/rethink-db-client">
                <img src="/assets/images/docs/driver-languages/objc.png" />
                <p class="name">Objective-C</p>
            </a>
            </li>
            <li>
            <a href="https://github.com/esycat/rql-scala">
                <img src="/assets/images/docs/driver-languages/scala.png" />
                <p class="name">Scala <span class="driver-author">esycat</span></p>
            </a>
            </li>
        </ul>
    </section>
</section>

{% infobox %}
    __Haven't installed the server yet?__ [Go do that](/install) first!
{% endinfobox %}

---

# Contribute a driver #

Help make RethinkDB available on more platforms&mdash;contribute a
driver for another language, or join one of the existing projects. To
get started with driver development:

- [Subscribe](https://groups.google.com/forum/?fromgroups=#!forum/rethinkdb-dev) to the RethinkDB driver developers group
- [Read](/docs/driver-spec/) the driver protocol specification
- [Browse](https://github.com/rethinkdb/rethinkdb/tree/v{{site.version.major}}.x/drivers) the source code for existing drivers

__Thanks to all our amazing driver contributors!__

- [@apa512](https://github.com/apa512) (Clojure): <https://github.com/apa512/clj-rethinkdb>
- [@atnnn](https://github.com/atnnn) (Haskell): [https://github.com/atnnn](https://github.com/atnnn/haskell-rethinkdb)
- [@bitemyapp](https://github.com/bitemyapp) and [@cesarbp](https://github.com/cesarbp) (Clojure): <https://github.com/bitemyapp/revise>
- [@christopherhesse](https://github.com/christopherhesse) (Go): <https://github.com/christopherhesse/rethinkgo>
- [@dancannon](https://github.com/dancannon) (Go): <https://github.com/dancannon/gorethink>
- [@danielmewes](https://github.com/danielmewes) (PHP): <https://github.com/danielmewes/php-rql>
- [@dbettin](https://github.com/dbettin) (Dart): <https://github.com/dbettin/rethinkdb>
- [@dkhenry](https://github.com/dkhenry) (Java): <https://github.com/dkhenry/rethinkjava>
- [@dparnell](https://github.com/dparnell) (Objective-C): <https://github.com/dparnell/rethink-db-client>
- [@esycat](https://github.com/esycat) (Scala): <https://github.com/esycat/rql-scala>
- [@grandquista](https://github.com/grandquista) (Lua): <https://github.com/grandquista/Lua-ReQL>
- [@jurajmasar](https://github.com/jurajmasar) (C++): <https://github.com/jurajmasar/rethink-db-cpp-driver>
- [@kclay](https://github.com/kclay) (Scala): <https://github.com/kclay/rethink-scala>
- [@neumino](https://github.com/neumino) (Node.js): <https://github.com/neumino/rethinkdbdash>
- [@njlg](https://github.com/njlg) (Perl): <https://github.com/njlg/perl-rethinkdb>
- [@npiv](https://github.com/npiv/) (Java): <https://github.com/npiv/rethink-java-driver>
- [@nuxlli](https://github.com/nuxlli) (Elixir): <https://github.com/azukiapp/elixir-rethinkdb>
- [@orthecreedence](https://github.com/orthecreedence) (Common Lisp): <https://github.com/orthecreedence/cl-rethinkdb>
- [@taybin](https://github.com/taybin) (Erlang): <https://github.com/taybin/lethink>
- [@unbit](https://github.com/unbit) (C): <https://github.com/unbit/librethinkdb>
- The [rethinkdb-net team](https://github.com/mfenniak/rethinkdb-net/graphs/contributors) (C# / .NET): <https://github.com/mfenniak/rethinkdb-net>
