# Rusty Kindle File Formats

Rust representations of data formats used by Kindle e-readers.
Currently only the KRDS format is supported. Documentation is given
when purpose of fields and types is known, if you know anything not
documented (or documented incorrectly) please contribute or make an
issue.

These representations may be innacurate. They may be missing some
fields and some fields may not be required by each format even though
I have not wrapped them in optionals. These representations may seem
strange, but they are designed to go with my other project
[serde_krds](https://github.com/willemml/serde_krds).

Information on format and value/field names were determined thanks to
users posting in [this mobilereads.com thread](https://www.mobileread.com/forums/showthread.php?t=322172).
