IUP Rust
========

[![Build Status](https://travis-ci.org/dcampbell24/iup-rust.svg)](https://travis-ci.org/dcampbell24/iup-rust)
[![Join the chat at https://gitter.im/dcampbell24/iup-rust](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/dcampbell24/iup-rust?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

This library provides a thin wrapper around [IUP][1], which is a multi-platform
toolkit for building graphical user interfaces. For a full overview of IUP see
the overview section on IUP's website

[1]: http://www.tecgraf.puc-rio.br/iup/

See the examples directory for examples of simple programs.

Features
--------

Currently, code written with library looks a lot like IUP C code, but this may
change in the future. The main goal right now is to allow people to create
simple GUI applications in Rust.

Although there are FFI bindings for most of the library, higher level wrappers
are only being created as the author needs them or other contributers add them.

Installation
------------

See the iup-sys [README.md](https://github.com/dcampbell24/iup-rust/tree/master/iup-sys)
for information on installing the IUP system libraries needed to use this
library.

Contribute
----------

Contributions are welcome both in the form of ideas for how to best wrap IUP in
Rust and high level wrappers for more of IUP. If you want to work on something,
please open a issue to let others know what you are working on.

Support
-------

If you find any issues with the library, please create a github issue for it.

License
-------

The project is licensed under the MIT license. See LICENSE for the full license.
