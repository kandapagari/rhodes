# Description

Rhodes is a Config manager writen in `rust` for `python`. This package can be used to load a `yaml` or `json` config file, using `rhodes.load('path/to/file.<yaml/yml/json>)`, This method returns a `dict` datatype in python.

# Limitation

- As of now this package can only read string key and value pairs in future all the data type will be supported.
- A wrapper od key `data` is needed right now to load the config file. This will be solved in future versions
