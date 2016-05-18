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
            <li>
                <a href="java/">
                    <img src="/assets/images/docs/driver-languages/java.png" />
                    <p class="name">Java</p>
                </a>
            </li>
        </ul>
    </section>
    
    <section class="platform-category">
        <h2>Current community-supported drivers</h2>
        <p>These drivers have been updated to use the JSON driver protocol and at least RethinkDB 2.0 ReQL.</p>
        <ul class="platform-buttons">
            <li>
                <a href="https://github.com/bchavez/RethinkDb.Driver">
                    <img src="/assets/images/docs/driver-languages/csharp.png" />
                    <p class="name">C# <span class="driver-author">bchavez</span></p>
                </a>
            </li>
            <li>
                <a href="https://github.com/mfenniak/rethinkdb-net">
                    <img src="/assets/images/docs/driver-languages/csharp.png" />
                    <p class="name">C# <span class="driver-author">mfenniak</span></p>
                </a>
            </li>
            <li>
                <a href="https://github.com/AtnNn/librethinkdbxx">
                    <img src="/assets/images/docs/driver-languages/cpp.png" />
                    <p class="name">C++</p>
                </a>
            </li>
            <li>
                <a href="https://github.com/apa512/clj-rethinkdb">
                    <img src="/assets/images/docs/driver-languages/clojure.png" />
                    <p class="name">Clojure</p>
                </a>
            </li>
            <li>
                <a href="https://github.com/orthecreedence/cl-rethinkdb">
                    <img src="/assets/images/docs/driver-languages/commonlisp.png" />
                    <p class="name smaller-font">Common Lisp</p>
                </a>
            </li>
            <li>
                <a href="https://github.com/billysometimes/rethinkdb">
                    <img src="/assets/images/docs/driver-languages/dart.png" />
                    <p class="name">Dart</p>
                </a>
            </li>
            <li>
                <a href="https://github.com/brandonhamilton/rethinkdb-delphi">
                    <img src="/assets/images/docs/driver-languages/delphi.png" />
                    <p class="name">Delphi</p>
                </a>
            </li>
            <li>
            <a href="https://github.com/hamiltop/rethinkdb-elixir">
                <img src="/assets/images/docs/driver-languages/elixir.png" />
                <p class="name">Elixir</p>
            </a>
            </li>
            <li>
                <a href="https://github.com/kureikain/relang">
                    <img src="/assets/images/docs/driver-languages/erlang.png" />
                    <p class="name">Erlang</p>
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
                <a href="https://github.com/rgv151/rethinkdb.nim">
                    <img src="/assets/images/docs/driver-languages/nim.png" />
                    <p class="name">Nim</p>
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
                <a href="https://github.com/mbq/rethinker">
                    <img src="/assets/images/docs/driver-languages/r.png" />
                    <p class="name">R</p>
                </a>
            </li>
            <li>
                <a href="https://github.com/pixelspark/rethink-swift">
                    <img src="/assets/images/docs/driver-languages/swift.png" />
                    <p class="name">Swift</p>
                </a>
            </li>
        </ul>
    </section>

    <section class="platform-category">
        <h2>Drivers with limited features</h2>
        <p>These drivers may not support all of RethinkDB 2.0's ReQL. However, if you wish to work with one of these languages, they may provide a good starting point.</p>
        <ul class="platform-buttons">
            <li>
                <a href="https://github.com/dparnell/rethink-db-client">
                    <img src="/assets/images/docs/driver-languages/objc.png" />
                    <p class="name">Objective-C</p>
                </a>
            </li>
            <li>
                <a href="https://github.com/kclay/rethink-scala">
                    <img src="/assets/images/docs/driver-languages/scala.png" />
                    <p class="name">Scala</p>
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

- Read the article on [writing RethinkDB drivers][wd].
- Subscribe to the [RethinkDB driver developers group][gg].
- Read the [source code for existing drivers][sc].

[wd]: /docs/writing-drivers/
[gg]: https://groups.google.com/forum/?fromgroups=#!forum/rethinkdb-dev
[sc]: https://github.com/rethinkdb/rethinkdb/tree/v{{site.version.major}}.x/drivers

__Thanks to all our amazing driver contributors!__

- [@apa512](https://github.com/apa512) (Clojure): <https://github.com/apa512/clj-rethinkdb>
- [@atnnn](https://github.com/atnnn) (Haskell): [https://github.com/atnnn](https://github.com/atnnn/haskell-rethinkdb)
- [@bchavez](https://github.com/bchavez) (C#/.NET): [https://github.com/bchavez/RethinkDb.Driver](https://github.com/bchavez/RethinkDb.Driver)
- [@billysometimes](https://github.com/billysometimes) (Dart): <https://github.com/billysometimes/rethinkdb>
- [@brandonhamilton](https://github.com/brandonhamilton) (Delphi): <https://github.com/brandonhamilton/rethinkdb-delphi>
- [@dancannon](https://github.com/dancannon) (Go): <https://github.com/dancannon/gorethink>
- [@danielmewes](https://github.com/danielmewes) (PHP): <https://github.com/danielmewes/php-rql>
- [@dkhenry](https://github.com/dkhenry) (Java): <https://github.com/dkhenry/rethinkjava>
- [@dparnell](https://github.com/dparnell) (Objective-C): <https://github.com/dparnell/rethink-db-client>
- [@grandquista](https://github.com/grandquista) (Lua): <https://github.com/grandquista/Lua-ReQL>
- [@hamiltop](https://github.com/hamiltop) (Elixir): <https://github.com/hamiltop/rethinkdb-elixir>
- [@jurajmasar](https://github.com/jurajmasar) (C++): <https://github.com/jurajmasar/rethink-db-cpp-driver>
- [@kclay](https://github.com/kclay) (Scala): <https://github.com/kclay/rethink-scala>
- [@kureikain](https://github.com/kureikain) (Erlang): <https://github.com/kureikain/relang>
- [@mbq](https://github.com/mbq) (R): <https://github.com/mbq/rethinker>
- [@neumino](https://github.com/neumino) (Node.js): <https://github.com/neumino/rethinkdbdash>
- [@njlg](https://github.com/njlg) (Perl): <https://github.com/njlg/perl-rethinkdb>
- [@npiv](https://github.com/npiv/) (Java): <https://github.com/npiv/rethink-java-driver>
- [@orthecreedence](https://github.com/orthecreedence) (Common Lisp): <https://github.com/orthecreedence/cl-rethinkdb>
- [@pixelspark](https://github.com/pixelspark) (Swift): <https://github.com/pixelspark/rethink-swift>
- [@rgv151](https://github.com/rgv151) (Nim): <https://github.com/rgv151/rethinkdb.nim>
- [@unbit](https://github.com/unbit) (C): <https://github.com/unbit/librethinkdb>
- The [rethinkdb-net team](https://github.com/mfenniak/rethinkdb-net/graphs/contributors) (C#/.NET): <https://github.com/mfenniak/rethinkdb-net>
