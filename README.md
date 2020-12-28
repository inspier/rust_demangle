# rust_demangle
Python wrapper around rustc-demangle.

## Installation

```python
$ pip install rust_demangle
```

## Usage

```python
import rust_demangle

print(rust_demangle.demangle("_ZN4testE")) # "test"
```
