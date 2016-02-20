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

Examples
--------

### Increment a Version Number

```bash
# 2.0.0
ver -1 1.2.3

# 1.3.0
ver -2 1.2.3

# 1.2.4
ver -3 1.2.3
```

### Comparing Two Version Numbers

```bash
# exit: 0
ver 1.2.3 -e 1.2.3
ver 1.2.3 -l 1.2.4
ver 1.2.4 -g 1.2.3

ver 1.2.3 -le 1.2.3
ver 1.2.0 -le 1.2.3

ver 1.2.3 -ge 1.2.3
ver 1.2.3 -ge 1.2.0

# exit: 1
ver 1.2.3 -e 2.2.3
ver 1.2.4 -l 1.2.3
ver 1.2.3 -g 1.2.4
```

Installation
------------

Install using Cargo.

    cargo install ver

License
-------

Released under the MIT license.

