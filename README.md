[![Build Status](https://travis-ci.org/kherge/ver.svg?branch=master)](https://travis-ci.org/kherge/ver)
[![Crates.io](https://img.shields.io/crates/v/ver.svg)](https://crates.io/crates/ver)

ver
===

Edit and compare semantic version numbers.

Usage
-----

```
Usage: ver [OPTIONS] VERSION [VERSION]
Edit and compare semantic version numbers.

Options:
    -1, --major         Increment the major version number.
    -2, --minor         Increment the minor version number.
    -3, --patch         Increment the patch version number.
    -e, --equal-to      Confirm VERSION is equal to VERSION.
    -g, --greater-than  Confirm VERSION is greater than VERSION.
    -h, --help          Displays this help mssage.
    -l, --less-than     Confirm VERSION is less than VERSION.
    -v, --version       Displays the version of this program.
```

Installation
------------

Install using Cargo.

    cargo install ver

License
-------

Released under the MIT license.

