# orjson

orjson is a fast, correct JSON library for Python. It
[benchmarks](https://github.com/ijl/orjson?tab=readme-ov-file#performance) as the fastest Python
library for JSON and is more correct than the standard json library or other
third-party libraries. It serializes
[dataclass](https://github.com/ijl/orjson?tab=readme-ov-file#dataclass),
[datetime](https://github.com/ijl/orjson?tab=readme-ov-file#datetime),
[numpy](https://github.com/ijl/orjson?tab=readme-ov-file#numpy), and
[UUID](https://github.com/ijl/orjson?tab=readme-ov-file#uuid) instances natively.

[orjson.dumps()](https://github.com/ijl/orjson?tab=readme-ov-file#serialize) is
something like 10x as fast as `json`, serializes
common types and subtypes, has a `default` parameter for the caller to specify
how to serialize arbitrary types, and has a number of flags controlling output.

[orjson.loads()](https://github.com/ijl/orjson?tab=readme-ov-file#deserialize)
is something like 2x as fast as `json`, and is strictly compliant with UTF-8 and
RFC 8259 ("The JavaScript Object Notation (JSON) Data Interchange Format").

Reading from and writing to files, line-delimited JSON files, and so on is
not provided by the library.

orjson supports CPython 3.8, 3.9, 3.10, 3.11, 3.12, 3.13, and 3.14.

It distributes amd64/x86_64, i686/x86, aarch64/armv8, arm7, POWER/ppc64le,
and s390x wheels for Linux, amd64 and aarch64 wheels for macOS, and amd64
and i686/x86 wheels for Windows.

orjson does not and will not support PyPy, embedded Python builds for
Android/iOS, or PEP 554 subinterpreters.

Releases follow semantic versioning and serializing a new object type
without an opt-in flag is considered a breaking change.

orjson is licensed under both the Apache 2.0 and MIT licenses. The
repository and issue tracker is
[github.com/ijl/orjson](https://github.com/ijl/orjson), and patches may be
submitted there. There is a
[CHANGELOG](https://github.com/ijl/orjson/blob/master/CHANGELOG.md)
available in the repository.

1. [Usage](https://github.com/ijl/orjson?tab=readme-ov-file#usage)
    1. [Install](https://github.com/ijl/orjson?tab=readme-ov-file#install)
    2. [Quickstart](https://github.com/ijl/orjson?tab=readme-ov-file#quickstart)
    3. [Migrating](https://github.com/ijl/orjson?tab=readme-ov-file#migrating)
    4. [Serialize](https://github.com/ijl/orjson?tab=readme-ov-file#serialize)
        1. [default](https://github.com/ijl/orjson?tab=readme-ov-file#default)
        2. [option](https://github.com/ijl/orjson?tab=readme-ov-file#option)
        3. [Fragment](https://github.com/ijl/orjson?tab=readme-ov-file#fragment)
    5. [Deserialize](https://github.com/ijl/orjson?tab=readme-ov-file#deserialize)
2. [Types](https://github.com/ijl/orjson?tab=readme-ov-file#types)
    1. [dataclass](https://github.com/ijl/orjson?tab=readme-ov-file#dataclass)
    2. [datetime](https://github.com/ijl/orjson?tab=readme-ov-file#datetime)
    3. [enum](https://github.com/ijl/orjson?tab=readme-ov-file#enum)
    4. [float](https://github.com/ijl/orjson?tab=readme-ov-file#float)
    5. [int](https://github.com/ijl/orjson?tab=readme-ov-file#int)
    6. [numpy](https://github.com/ijl/orjson?tab=readme-ov-file#numpy)
    7. [str](https://github.com/ijl/orjson?tab=readme-ov-file#str)
    8. [uuid](https://github.com/ijl/orjson?tab=readme-ov-file#uuid)
3. [Testing](https://github.com/ijl/orjson?tab=readme-ov-file#testing)
4. [Performance](https://github.com/ijl/orjson?tab=readme-ov-file#performance)
    1. [Latency](https://github.com/ijl/orjson?tab=readme-ov-file#latency)
    2. [Reproducing](https://github.com/ijl/orjson?tab=readme-ov-file#reproducing)
5. [Questions](https://github.com/ijl/orjson?tab=readme-ov-file#questions)
6. [Packaging](https://github.com/ijl/orjson?tab=readme-ov-file#packaging)
7. [License](https://github.com/ijl/orjson?tab=readme-ov-file#license)

## Usage

### Install

To install a wheel from PyPI, install the `orjson` package.

In `requirements.in` or `requirements.txt` format, specify:

```txt
orjson >= 3.10,<4
```

In `pyproject.toml` format, specify:

```toml
orjson = "^3.10"
```

To build a wheel, see [packaging](https://github.com/ijl/orjson?tab=readme-ov-file#packaging).

### Quickstart

This is an example of serializing, with options specified, and deserializing:

```python
>>> import orjson, datetime, numpy
>>> data = {
    "type": "job",
    "created_at": datetime.datetime(1970, 1, 1),
    "status": "🆗",
    "payload": numpy.array([[1, 2], [3, 4]]),
}
>>> orjson.dumps(data, option=orjson.OPT_NAIVE_UTC | orjson.OPT_SERIALIZE_NUMPY)
b'{"type":"job","created_at":"1970-01-01T00:00:00+00:00","status":"\xf0\x9f\x86\x97","payload":[[1,2],[3,4]]}'
>>> orjson.loads(_)
{'type': 'job', 'created_at': '1970-01-01T00:00:00+00:00', 'status': '🆗', 'payload': [[1, 2], [3, 4]]}
```

### Migrating

orjson version 3 serializes more types than version 2. Subclasses of `str`,
`int`, `dict`, and `list` are now serialized. This is faster and more similar
to the standard library. It can be disabled with
`orjson.OPT_PASSTHROUGH_SUBCLASS`.`dataclasses.dataclass` instances
are now serialized by default and cannot be customized in a
`default` function unless `option=orjson.OPT_PASSTHROUGH_DATACLASS` is
specified. `uuid.UUID` instances are serialized by default.
For any type that is now serialized,
implementations in a `default` function and options enabling them can be
removed but do not need to be. There was no change in deserialization.

To migrate from the standard library, the largest difference is that
`orjson.dumps` returns `bytes` and `json.dumps` returns a `str`.

Users with `dict` objects using non-`str` keys should specify `option=orjson.OPT_NON_STR_KEYS`.

`sort_keys` is replaced by `option=orjson.OPT_SORT_KEYS`.

`indent` is replaced by `option=orjson.OPT_INDENT_2` and other levels of indentation are not
supported.

`ensure_ascii` is probably not relevant today and UTF-8 characters cannot be
escaped to ASCII.

### Serialize

```python
def dumps(
    __obj: Any,
    default: Optional[Callable[[Any], Any]] = ...,
    option: Optional[int] = ...,
) -> bytes: ...
```

`dumps()` serializes Python objects to JSON.

It natively serializes
`str`, `dict`, `list`, `tuple`, `int`, `float`, `bool`, `None`,
`dataclasses.dataclass`, `typing.TypedDict`, `datetime.datetime`,
`datetime.date`, `datetime.time`, `uuid.UUID`, `numpy.ndarray`, and
`orjson.Fragment` instances. It supports arbitrary types through `default`. It
serializes subclasses of `str`, `int`, `dict`, `list`,
`dataclasses.dataclass`, and `enum.Enum`. It does not serialize subclasses
of `tuple` to avoid serializing `namedtuple` objects as arrays. To avoid
serializing subclasses, specify the option `orjson.OPT_PASSTHROUGH_SUBCLASS`.

The output is a `bytes` object containing UTF-8.

The global interpreter lock (GIL) is held for the duration of the call.

It raises `JSONEncodeError` on an unsupported type. This exception message
describes the invalid object with the error message
`Type is not JSON serializable: ...`. To fix this, specify
[default](https://github.com/ijl/orjson?tab=readme-ov-file#default).

It raises `JSONEncodeError` on a `str` that contains invalid UTF-8.

It raises `JSONEncodeError` on an integer that exceeds 64 bits by default or,
with `OPT_STRICT_INTEGER`, 53 bits.

It raises `JSONEncodeError` if a `dict` has a key of a type other than `str`,
unless `OPT_NON_STR_KEYS` is specified.

It raises `JSONEncodeError` if the output of `default` recurses to handling by
`default` more than 254 levels deep.

It raises `JSONEncodeError` on circular references.

It raises `JSONEncodeError`  if a `tzinfo` on a datetime object is
unsupported.

`JSONEncodeError` is a subclass of `TypeError`. This is for compatibility
with the standard library.

If the failure was caused by an exception in `default` then
`JSONEncodeError` chains the original exception as `__cause__`.

#### default

To serialize a subclass or arbitrary types, specify `default` as a
callable that returns a supported type. `default` may be a function,
lambda, or callable class instance. To specify that a type was not
handled by `default`, raise an exception such as `TypeError`.

```python
>>> import orjson, decimal
>>>
def default(obj):
    if isinstance(obj, decimal.Decimal):
        return str(obj)
    raise TypeError

>>> orjson.dumps(decimal.Decimal("0.0842389659712649442845"))
JSONEncodeError: Type is not JSON serializable: decimal.Decimal
>>> orjson.dumps(decimal.Decimal("0.0842389659712649442845"), default=default)
b'"0.0842389659712649442845"'
>>> orjson.dumps({1, 2}, default=default)
orjson.JSONEncodeError: Type is not JSON serializable: set
```

The `default` callable may return an object that itself
must be handled by `default` up to 254 times before an exception
is raised.

It is important that `default` raise an exception if a type cannot be handled.
Python otherwise implicitly returns `None`, which appears to the caller
like a legitimate value and is serialized:

```python
>>> import orjson, json
>>>
def default(obj):
    if isinstance(obj, decimal.Decimal):
        return str(obj)

>>> orjson.dumps({"set":{1, 2}}, default=default)
b'{"set":null}'
>>> json.dumps({"set":{1, 2}}, default=default)
'{"set":null}'
```

#### option

To modify how data is serialized, specify `option`. Each `option` is an integer
constant in `orjson`. To specify multiple options, mask them together, e.g.,
`option=orjson.OPT_STRICT_INTEGER | orjson.OPT_NAIVE_UTC`.

##### OPT_APPEND_NEWLINE

Append `\n` to the output. This is a convenience and optimization for the
pattern of `dumps(...) + "\n"`. `bytes` objects are immutable and this
pattern copies the original contents.

```python
>>> import orjson
>>> orjson.dumps([])
b"[]"
>>> orjson.dumps([], option=orjson.OPT_APPEND_NEWLINE)
b"[]\n"
```

##### OPT_INDENT_2

Pretty-print output with an indent of two spaces. This is equivalent to
`indent=2` in the standard library. Pretty printing is slower and the output
larger. orjson is the fastest compared library at pretty printing and has
much less of a slowdown to pretty print than the standard library does. This
option is compatible with all other options.

```python
>>> import orjson
>>> orjson.dumps({"a": "b", "c": {"d": True}, "e": [1, 2]})
b'{"a":"b","c":{"d":true},"e":[1,2]}'
>>> orjson.dumps(
    {"a": "b", "c": {"d": True}, "e": [1, 2]},
    option=orjson.OPT_INDENT_2
)
b'{\n  "a": "b",\n  "c": {\n    "d": true\n  },\n  "e": [\n    1,\n    2\n  ]\n}'
```

If displayed, the indentation and linebreaks appear like this:

```json
{
  "a": "b",
  "c": {
    "d": true
  },
  "e": [
    1,
    2
  ]
}
```

This measures serializing the github.json fixture as compact (52KiB) or
pretty (64KiB):

| Library   |   compact (ms) |   pretty (ms) |   vs. orjson |
|-----------|----------------|---------------|--------------|
| orjson    |           0.01 |          0.02 |            1 |
| json      |           0.13 |          0.54 |           34 |

This measures serializing the citm_catalog.json fixture, more of a worst
case due to the amount of nesting and newlines, as compact (489KiB) or
pretty (1.1MiB):

| Library   |   compact (ms) |   pretty (ms) |   vs. orjson |
|-----------|----------------|---------------|--------------|
| orjson    |           0.25 |          0.45 |          1   |
| json      |           3.01 |         24.42 |         54.4 |

This can be reproduced using the `pyindent` script.

##### OPT_NAIVE_UTC

Serialize `datetime.datetime` objects without a `tzinfo` as UTC. This
has no effect on `datetime.datetime` objects that have `tzinfo` set.

```python
>>> import orjson, datetime
>>> orjson.dumps(
        datetime.datetime(1970, 1, 1, 0, 0, 0),
    )
b'"1970-01-01T00:00:00"'
>>> orjson.dumps(
        datetime.datetime(1970, 1, 1, 0, 0, 0),
        option=orjson.OPT_NAIVE_UTC,
    )
b'"1970-01-01T00:00:00+00:00"'
```

##### OPT_NON_STR_KEYS

Serialize `dict` keys of type other than `str`. This allows `dict` keys
to be one of `str`, `int`, `float`, `bool`, `None`, `datetime.datetime`,
`datetime.date`, `datetime.time`, `enum.Enum`, and `uuid.UUID`. For comparison,
the standard library serializes `str`, `int`, `float`, `bool` or `None` by
default. orjson benchmarks as being faster at serializing non-`str` keys
than other libraries. This option is slower for `str` keys than the default.

```python
>>> import orjson, datetime, uuid
>>> orjson.dumps(
        {uuid.UUID("7202d115-7ff3-4c81-a7c1-2a1f067b1ece"): [1, 2, 3]},
        option=orjson.OPT_NON_STR_KEYS,
    )
b'{"7202d115-7ff3-4c81-a7c1-2a1f067b1ece":[1,2,3]}'
>>> orjson.dumps(
        {datetime.datetime(1970, 1, 1, 0, 0, 0): [1, 2, 3]},
        option=orjson.OPT_NON_STR_KEYS | orjson.OPT_NAIVE_UTC,
    )
b'{"1970-01-01T00:00:00+00:00":[1,2,3]}'
```

These types are generally serialized how they would be as
values, e.g., `datetime.datetime` is still an RFC 3339 string and respects
options affecting it. The exception is that `int` serialization does not
respect `OPT_STRICT_INTEGER`.

This option has the risk of creating duplicate keys. This is because non-`str`
objects may serialize to the same `str` as an existing key, e.g.,
`{"1": true, 1: false}`. The last key to be inserted to the `dict` will be
serialized last and a JSON deserializer will presumably take the last
occurrence of a key (in the above, `false`). The first value will be lost.

This option is compatible with `orjson.OPT_SORT_KEYS`. If sorting is used,
note the sort is unstable and will be unpredictable for duplicate keys.

```python
>>> import orjson, datetime
>>> orjson.dumps(
    {"other": 1, datetime.date(1970, 1, 5): 2, datetime.date(1970, 1, 3): 3},
    option=orjson.OPT_NON_STR_KEYS | orjson.OPT_SORT_KEYS
)
b'{"1970-01-03":3,"1970-01-05":2,"other":1}'
```

This measures serializing 589KiB of JSON comprising a `list` of 100 `dict`
in which each `dict` has both 365 randomly-sorted `int` keys representing epoch
timestamps as well as one `str` key and the value for each key is a
single integer. In "str keys", the keys were converted to `str` before
serialization, and orjson still specifes `option=orjson.OPT_NON_STR_KEYS`
(which is always somewhat slower).

| Library   |   str keys (ms) |   int keys (ms) | int keys sorted (ms)   |
|-----------|-----------------|-----------------|------------------------|
| orjson    |            0.5  |            0.93 | 2.08                   |
| json      |            2.72 |            3.59 |                        |

json is blank because it
raises `TypeError` on attempting to sort before converting all keys to `str`.
This can be reproduced using the `pynonstr` script.

##### OPT_OMIT_MICROSECONDS

Do not serialize the `microsecond` field on `datetime.datetime` and
`datetime.time` instances.

```python
>>> import orjson, datetime
>>> orjson.dumps(
        datetime.datetime(1970, 1, 1, 0, 0, 0, 1),
    )
b'"1970-01-01T00:00:00.000001"'
>>> orjson.dumps(
        datetime.datetime(1970, 1, 1, 0, 0, 0, 1),
        option=orjson.OPT_OMIT_MICROSECONDS,
    )
b'"1970-01-01T00:00:00"'
```

##### OPT_PASSTHROUGH_DATACLASS

Passthrough `dataclasses.dataclass` instances to `default`. This allows
customizing their output but is much slower.


```python
>>> import orjson, dataclasses
>>>
@dataclasses.dataclass
class User:
    id: str
    name: str
    password: str

def default(obj):
    if isinstance(obj, User):
        return {"id": obj.id, "name": obj.name}
    raise TypeError

>>> orjson.dumps(User("3b1", "asd", "zxc"))
b'{"id":"3b1","name":"asd","password":"zxc"}'
>>> orjson.dumps(User("3b1", "asd", "zxc"), option=orjson.OPT_PASSTHROUGH_DATACLASS)
TypeError: Type is not JSON serializable: User
>>> orjson.dumps(
        User("3b1", "asd", "zxc"),
        option=orjson.OPT_PASSTHROUGH_DATACLASS,
        default=default,
    )
b'{"id":"3b1","name":"asd"}'
```

##### OPT_PASSTHROUGH_DATETIME

Passthrough `datetime.datetime`, `datetime.date`, and `datetime.time` instances
to `default`. This allows serializing datetimes to a custom format, e.g.,
HTTP dates:

```python
>>> import orjson, datetime
>>>
def default(obj):
    if isinstance(obj, datetime.datetime):
        return obj.strftime("%a, %d %b %Y %H:%M:%S GMT")
    raise TypeError

>>> orjson.dumps({"created_at": datetime.datetime(1970, 1, 1)})
b'{"created_at":"1970-01-01T00:00:00"}'
>>> orjson.dumps({"created_at": datetime.datetime(1970, 1, 1)}, option=orjson.OPT_PASSTHROUGH_DATETIME)
TypeError: Type is not JSON serializable: datetime.datetime
>>> orjson.dumps(
        {"created_at": datetime.datetime(1970, 1, 1)},
        option=orjson.OPT_PASSTHROUGH_DATETIME,
        default=default,
    )
b'{"created_at":"Thu, 01 Jan 1970 00:00:00 GMT"}'
```

This does not affect datetimes in `dict` keys if using OPT_NON_STR_KEYS.

##### OPT_PASSTHROUGH_SUBCLASS

Passthrough subclasses of builtin types to `default`.

```python
>>> import orjson
>>>
class Secret(str):
    pass

def default(obj):
    if isinstance(obj, Secret):
        return "******"
    raise TypeError

>>> orjson.dumps(Secret("zxc"))
b'"zxc"'
>>> orjson.dumps(Secret("zxc"), option=orjson.OPT_PASSTHROUGH_SUBCLASS)
TypeError: Type is not JSON serializable: Secret
>>> orjson.dumps(Secret("zxc"), option=orjson.OPT_PASSTHROUGH_SUBCLASS, default=default)
b'"******"'
```

This does not affect serializing subclasses as `dict` keys if using
OPT_NON_STR_KEYS.

##### OPT_SERIALIZE_DATACLASS

This is deprecated and has no effect in version 3. In version 2 this was
required to serialize  `dataclasses.dataclass` instances. For more, see
[dataclass](https://github.com/ijl/orjson?tab=readme-ov-file#dataclass).

##### OPT_SERIALIZE_NUMPY

Serialize `numpy.ndarray` instances. For more, see
[numpy](https://github.com/ijl/orjson?tab=readme-ov-file#numpy).

##### OPT_SERIALIZE_UUID

This is deprecated and has no effect in version 3. In version 2 this was
required to serialize `uuid.UUID` instances. For more, see
[UUID](https://github.com/ijl/orjson?tab=readme-ov-file#UUID).

##### OPT_SORT_KEYS

Serialize `dict` keys in sorted order. The default is to serialize in an
unspecified order. This is equivalent to `sort_keys=True` in the standard
library.

This can be used to ensure the order is deterministic for hashing or tests.
It has a substantial performance penalty and is not recommended in general.

```python
>>> import orjson
>>> orjson.dumps({"b": 1, "c": 2, "a": 3})
b'{"b":1,"c":2,"a":3}'
>>> orjson.dumps({"b": 1, "c": 2, "a": 3}, option=orjson.OPT_SORT_KEYS)
b'{"a":3,"b":1,"c":2}'
```

This measures serializing the twitter.json fixture unsorted and sorted:

| Library   |   unsorted (ms) |   sorted (ms) |   vs. orjson |
|-----------|-----------------|---------------|--------------|
| orjson    |            0.11 |          0.3  |          1   |
| json      |            1.36 |          1.93 |          6.4 |

The benchmark can be reproduced using the `pysort` script.

The sorting is not collation/locale-aware:

```python
>>> import orjson
>>> orjson.dumps({"a": 1, "ä": 2, "A": 3}, option=orjson.OPT_SORT_KEYS)
b'{"A":3,"a":1,"\xc3\xa4":2}'
```

This is the same sorting behavior as the standard library.

`dataclass` also serialize as maps but this has no effect on them.

##### OPT_STRICT_INTEGER

Enforce 53-bit limit on integers. The limit is otherwise 64 bits, the same as
the Python standard library. For more, see [int](https://github.com/ijl/orjson?tab=readme-ov-file#int).

##### OPT_UTC_Z

Serialize a UTC timezone on `datetime.datetime` instances as `Z` instead
of `+00:00`.

```python
>>> import orjson, datetime, zoneinfo
>>> orjson.dumps(
        datetime.datetime(1970, 1, 1, 0, 0, 0, tzinfo=zoneinfo.ZoneInfo("UTC")),
    )
b'"1970-01-01T00:00:00+00:00"'
>>> orjson.dumps(
        datetime.datetime(1970, 1, 1, 0, 0, 0, tzinfo=zoneinfo.ZoneInfo("UTC")),
        option=orjson.OPT_UTC_Z
    )
b'"1970-01-01T00:00:00Z"'
```

#### Fragment

`orjson.Fragment` includes already-serialized JSON in a document. This is an
efficient way to include JSON blobs from a cache, JSONB field, or separately
serialized object without first deserializing to Python objects via `loads()`.

```python
>>> import orjson
>>> orjson.dumps({"key": "zxc", "data": orjson.Fragment(b'{"a": "b", "c": 1}')})
b'{"key":"zxc","data":{"a": "b", "c": 1}}'
```

It does no reformatting: `orjson.OPT_INDENT_2` will not affect a
compact blob nor will a pretty-printed JSON blob be rewritten as compact.

The input must be `bytes` or `str` and given as a positional argument.

This raises `orjson.JSONEncodeError` if a `str` is given and the input is
not valid UTF-8. It otherwise does no validation and it is possible to
write invalid JSON. This does not escape characters. The implementation is
tested to not crash if given invalid strings or invalid JSON.

### Deserialize

```python
def loads(__obj: Union[bytes, bytearray, memoryview, str]) -> Any: ...
```

`loads()` deserializes JSON to Python objects. It deserializes to `dict`,
`list`, `int`, `float`, `str`, `bool`, and `None` objects.

`bytes`, `bytearray`, `memoryview`, and `str` input are accepted. If the input
exists as a `memoryview`, `bytearray`, or `bytes` object, it is recommended to
pass these directly rather than creating an unnecessary `str` object. That is,
`orjson.loads(b"{}")` instead of `orjson.loads(b"{}".decode("utf-8"))`. This
has lower memory usage and lower latency.

The input must be valid UTF-8.

orjson maintains a cache of map keys for the duration of the process. This
causes a net reduction in memory usage by avoiding duplicate strings. The
keys must be at most 64 bytes to be cached and 2048 entries are stored.

The global interpreter lock (GIL) is held for the duration of the call.

It raises `JSONDecodeError` if given an invalid type or invalid
JSON. This includes if the input contains `NaN`, `Infinity`, or `-Infinity`,
which the standard library allows, but is not valid JSON.

It raises `JSONDecodeError` if a combination of array or object recurses
1024 levels deep.

`JSONDecodeError` is a subclass of `json.JSONDecodeError` and `ValueError`.
This is for compatibility with the standard library.

## Types

### dataclass

orjson serializes instances of `dataclasses.dataclass` natively. It serializes
instances 40-50x as fast as other libraries and avoids a severe slowdown seen
in other libraries compared to serializing `dict`.

It is supported to pass all variants of dataclasses, including dataclasses
using `__slots__`, frozen dataclasses, those with optional or default
attributes, and subclasses. There is a performance benefit to not
using `__slots__`.

| Library   |   dict (ms) |   dataclass (ms) |   vs. orjson |
|-----------|-------------|------------------|--------------|
| orjson    |        0.43 |             0.95 |            1 |
| json      |        5.81 |            38.32 |           40 |

This measures serializing 555KiB of JSON, orjson natively and other libraries
using `default` to serialize the output of `dataclasses.asdict()`. This can be
reproduced using the `pydataclass` script.

Dataclasses are serialized as maps, with every attribute serialized and in
the order given on class definition:

```python
>>> import dataclasses, orjson, typing

@dataclasses.dataclass
class Member:
    id: int
    active: bool = dataclasses.field(default=False)

@dataclasses.dataclass
class Object:
    id: int
    name: str
    members: typing.List[Member]

>>> orjson.dumps(Object(1, "a", [Member(1, True), Member(2)]))
b'{"id":1,"name":"a","members":[{"id":1,"active":true},{"id":2,"active":false}]}'
```

### datetime

orjson serializes `datetime.datetime` objects to
[RFC 3339](https://tools.ietf.org/html/rfc3339) format,
e.g., "1970-01-01T00:00:00+00:00". This is a subset of ISO 8601 and is
compatible with `isoformat()` in the standard library.

```python
>>> import orjson, datetime, zoneinfo
>>> orjson.dumps(
    datetime.datetime(2018, 12, 1, 2, 3, 4, 9, tzinfo=zoneinfo.ZoneInfo("Australia/Adelaide"))
)
b'"2018-12-01T02:03:04.000009+10:30"'
>>> orjson.dumps(
    datetime.datetime(2100, 9, 1, 21, 55, 2).replace(tzinfo=zoneinfo.ZoneInfo("UTC"))
)
b'"2100-09-01T21:55:02+00:00"'
>>> orjson.dumps(
    datetime.datetime(2100, 9, 1, 21, 55, 2)
)
b'"2100-09-01T21:55:02"'
```

`datetime.datetime` supports instances with a `tzinfo` that is `None`,
`datetime.timezone.utc`, a timezone instance from the python3.9+ `zoneinfo`
module, or a timezone instance from the third-party `pendulum`, `pytz`, or
`dateutil`/`arrow` libraries.

It is fastest to use the standard library's `zoneinfo.ZoneInfo` for timezones.

`datetime.time` objects must not have a `tzinfo`.

```python
>>> import orjson, datetime
>>> orjson.dumps(datetime.time(12, 0, 15, 290))
b'"12:00:15.000290"'
```

`datetime.date` objects will always serialize.

```python
>>> import orjson, datetime
>>> orjson.dumps(datetime.date(1900, 1, 2))
b'"1900-01-02"'
```

Errors with `tzinfo` result in `JSONEncodeError` being raised.

To disable serialization of `datetime` objects specify the option
`orjson.OPT_PASSTHROUGH_DATETIME`.

To use "Z" suffix instead of "+00:00" to indicate UTC ("Zulu") time, use the option
`orjson.OPT_UTC_Z`.

To assume datetimes without timezone are UTC, use the option `orjson.OPT_NAIVE_UTC`.

### enum

orjson serializes enums natively. Options apply to their values.

```python
>>> import enum, datetime, orjson
>>>
class DatetimeEnum(enum.Enum):
    EPOCH = datetime.datetime(1970, 1, 1, 0, 0, 0)
>>> orjson.dumps(DatetimeEnum.EPOCH)
b'"1970-01-01T00:00:00"'
>>> orjson.dumps(DatetimeEnum.EPOCH, option=orjson.OPT_NAIVE_UTC)
b'"1970-01-01T00:00:00+00:00"'
```

Enums with members that are not supported types can be serialized using
`default`:

```python
>>> import enum, orjson
>>>
class Custom:
    def __init__(self, val):
        self.val = val

def default(obj):
    if isinstance(obj, Custom):
        return obj.val
    raise TypeError

class CustomEnum(enum.Enum):
    ONE = Custom(1)

>>> orjson.dumps(CustomEnum.ONE, default=default)
b'1'
```

### float

orjson serializes and deserializes double precision floats with no loss of
precision and consistent rounding.

`orjson.dumps()` serializes Nan, Infinity, and -Infinity, which are not
compliant JSON, as `null`:

```python
>>> import orjson, json
>>> orjson.dumps([float("NaN"), float("Infinity"), float("-Infinity")])
b'[null,null,null]'
>>> json.dumps([float("NaN"), float("Infinity"), float("-Infinity")])
'[NaN, Infinity, -Infinity]'
```

### int

orjson serializes and deserializes 64-bit integers by default. The range
supported is a signed 64-bit integer's minimum (-9223372036854775807) to
an unsigned 64-bit integer's maximum (18446744073709551615). This
is widely compatible, but there are implementations
that only support 53-bits for integers, e.g.,
web browsers. For those implementations, `dumps()` can be configured to
raise a `JSONEncodeError` on values exceeding the 53-bit range.

```python
>>> import orjson
>>> orjson.dumps(9007199254740992)
b'9007199254740992'
>>> orjson.dumps(9007199254740992, option=orjson.OPT_STRICT_INTEGER)
JSONEncodeError: Integer exceeds 53-bit range
>>> orjson.dumps(-9007199254740992, option=orjson.OPT_STRICT_INTEGER)
JSONEncodeError: Integer exceeds 53-bit range
```

### numpy

orjson natively serializes `numpy.ndarray` and individual
`numpy.float64`, `numpy.float32`, `numpy.float16` (`numpy.half`),
`numpy.int64`, `numpy.int32`, `numpy.int16`, `numpy.int8`,
`numpy.uint64`, `numpy.uint32`, `numpy.uint16`, `numpy.uint8`,
`numpy.uintp`, `numpy.intp`, `numpy.datetime64`, and `numpy.bool`
instances.

orjson is compatible with both numpy v1 and v2.

orjson is faster than all compared libraries at serializing
numpy instances. Serializing numpy data requires specifying
`option=orjson.OPT_SERIALIZE_NUMPY`.

```python
>>> import orjson, numpy
>>> orjson.dumps(
        numpy.array([[1, 2, 3], [4, 5, 6]]),
        option=orjson.OPT_SERIALIZE_NUMPY,
)
b'[[1,2,3],[4,5,6]]'
```

The array must be a contiguous C array (`C_CONTIGUOUS`) and one of the
supported datatypes.

Note a difference between serializing `numpy.float32` using `ndarray.tolist()`
or `orjson.dumps(..., option=orjson.OPT_SERIALIZE_NUMPY)`: `tolist()` converts
to a `double` before serializing and orjson's native path does not. This
can result in different rounding.

`numpy.datetime64` instances are serialized as RFC 3339 strings and
datetime options affect them.

```python
>>> import orjson, numpy
>>> orjson.dumps(
        numpy.datetime64("2021-01-01T00:00:00.172"),
        option=orjson.OPT_SERIALIZE_NUMPY,
)
b'"2021-01-01T00:00:00.172000"'
>>> orjson.dumps(
        numpy.datetime64("2021-01-01T00:00:00.172"),
        option=(
            orjson.OPT_SERIALIZE_NUMPY |
            orjson.OPT_NAIVE_UTC |
            orjson.OPT_OMIT_MICROSECONDS
        ),
)
b'"2021-01-01T00:00:00+00:00"'
```

If an array is not a contiguous C array, contains an unsupported datatype,
or contains a `numpy.datetime64` using an unsupported representation
(e.g., picoseconds), orjson falls through to `default`. In `default`,
`obj.tolist()` can be specified.

If an array is not in the native endianness, e.g., an array of big-endian values
on a little-endian system, `orjson.JSONEncodeError`  is raised.

If an array is malformed, `orjson.JSONEncodeError` is raised.

This measures serializing 92MiB of JSON from an `numpy.ndarray` with
dimensions of `(50000, 100)` and `numpy.float64` values:

| Library   | Latency (ms)   |   RSS diff (MiB) |   vs. orjson |
|-----------|----------------|------------------|--------------|
| orjson    | 105            |              105 |          1   |
| json      | 1,481          |              295 |         14.2 |

This measures serializing 100MiB of JSON from an `numpy.ndarray` with
dimensions of `(100000, 100)` and `numpy.int32` values:

| Library   |   Latency (ms) |   RSS diff (MiB) |   vs. orjson |
|-----------|----------------|------------------|--------------|
| orjson    |             68 |              119 |          1   |
| json      |            684 |              501 |         10.1 |

This measures serializing 105MiB of JSON from an `numpy.ndarray` with
dimensions of `(100000, 200)` and `numpy.bool` values:

| Library   |   Latency (ms) |   RSS diff (MiB) |   vs. orjson |
|-----------|----------------|------------------|--------------|
| orjson    |             50 |              125 |          1   |
| json      |            573 |              398 |         11.5 |

In these benchmarks, orjson serializes natively and `json` serializes
`ndarray.tolist()` via `default`. The RSS column measures peak memory
usage during serialization. This can be reproduced using the `pynumpy` script.

orjson does not have an installation or compilation dependency on numpy. The
implementation is independent, reading `numpy.ndarray` using
`PyArrayInterface`.

### str

orjson is strict about UTF-8 conformance. This is stricter than the standard
library's json module, which will serialize and deserialize UTF-16 surrogates,
e.g., "\ud800", that are invalid UTF-8.

If `orjson.dumps()` is given a `str` that does not contain valid UTF-8,
`orjson.JSONEncodeError` is raised. If `loads()` receives invalid UTF-8,
`orjson.JSONDecodeError` is raised.

```python
>>> import orjson, json
>>> orjson.dumps('\ud800')
JSONEncodeError: str is not valid UTF-8: surrogates not allowed
>>> json.dumps('\ud800')
'"\\ud800"'
>>> orjson.loads('"\\ud800"')
JSONDecodeError: unexpected end of hex escape at line 1 column 8: line 1 column 1 (char 0)
>>> json.loads('"\\ud800"')
'\ud800'
```

To make a best effort at deserializing bad input, first decode `bytes` using
the `replace` or `lossy` argument for `errors`:

```python
>>> import orjson
>>> orjson.loads(b'"\xed\xa0\x80"')
JSONDecodeError: str is not valid UTF-8: surrogates not allowed
>>> orjson.loads(b'"\xed\xa0\x80"'.decode("utf-8", "replace"))
'���'
```

### uuid

orjson serializes `uuid.UUID` instances to
[RFC 4122](https://tools.ietf.org/html/rfc4122) format, e.g.,
"f81d4fae-7dec-11d0-a765-00a0c91e6bf6".

``` python
>>> import orjson, uuid
>>> orjson.dumps(uuid.uuid5(uuid.NAMESPACE_DNS, "python.org"))
b'"886313e1-3b8a-5372-9b90-0c9aee199e5d"'
```

## Testing

The library has comprehensive tests. There are tests against fixtures in the
[JSONTestSuite](https://github.com/nst/JSONTestSuite) and
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark)
repositories. It is tested to not crash against the
[Big List of Naughty Strings](https://github.com/minimaxir/big-list-of-naughty-strings).
It is tested to not leak memory. It is tested to not crash
against and not accept invalid UTF-8. There are integration tests
exercising the library's use in web servers (gunicorn using multiprocess/forked
workers) and when
multithreaded. It also uses some tests from the ultrajson library.

orjson is the most correct of the compared libraries. This graph shows how each
library handles a combined 342 JSON fixtures from the
[JSONTestSuite](https://github.com/nst/JSONTestSuite) and
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark) tests:

| Library    |   Invalid JSON documents not rejected |   Valid JSON documents not deserialized |
|------------|---------------------------------------|-----------------------------------------|
| orjson     |                                     0 |                                       0 |
| json       |                                    17 |                                       0 |

This shows that all libraries deserialize valid JSON but only orjson
correctly rejects the given invalid JSON fixtures. Errors are largely due to
accepting invalid strings and numbers.

The graph above can be reproduced using the `pycorrectness` script.

## Performance

Serialization and deserialization performance of orjson is consistently better
than the standard library's `json`. The graphs below illustrate a few commonly
used documents.

### Latency

![Serialization](doc/serialization.png)

![Deserialization](doc/deserialization.png)

#### twitter.json serialization

| Library   |   Median latency (milliseconds) |   Operations per second |   Relative (latency) |
|-----------|---------------------------------|-------------------------|----------------------|
| orjson    |                             0.1 |                    8453 |                  1   |
| json      |                             1.3 |                     765 |                 11.1 |

#### twitter.json deserialization

| Library   |   Median latency (milliseconds) |   Operations per second |   Relative (latency) |
|-----------|---------------------------------|-------------------------|----------------------|
| orjson    |                             0.5 |                    1889 |                  1   |
| json      |                             2.2 |                     453 |                  4.2 |

#### github.json serialization

| Library   |   Median latency (milliseconds) |   Operations per second |   Relative (latency) |
|-----------|---------------------------------|-------------------------|----------------------|
| orjson    |                            0.01 |                  103693 |                  1   |
| json      |                            0.13 |                    7648 |                 13.6 |

#### github.json deserialization

| Library   |   Median latency (milliseconds) |   Operations per second |   Relative (latency) |
|-----------|---------------------------------|-------------------------|----------------------|
| orjson    |                            0.04 |                   23264 |                  1   |
| json      |                            0.1  |                   10430 |                  2.2 |

#### citm_catalog.json serialization

| Library   |   Median latency (milliseconds) |   Operations per second |   Relative (latency) |
|-----------|---------------------------------|-------------------------|----------------------|
| orjson    |                             0.3 |                    3975 |                  1   |
| json      |                             3   |                     338 |                 11.8 |

#### citm_catalog.json deserialization

| Library   |   Median latency (milliseconds) |   Operations per second |   Relative (latency) |
|-----------|---------------------------------|-------------------------|----------------------|
| orjson    |                             1.3 |                     781 |                  1   |
| json      |                             4   |                     250 |                  3.1 |

#### canada.json serialization

| Library   |   Median latency (milliseconds) |   Operations per second |   Relative (latency) |
|-----------|---------------------------------|-------------------------|----------------------|
| orjson    |                             2.5 |                     399 |                  1   |
| json      |                            29.8 |                      33 |                 11.9 |

#### canada.json deserialization

| Library   |   Median latency (milliseconds) |   Operations per second |   Relative (latency) |
|-----------|---------------------------------|-------------------------|----------------------|
| orjson    |                               3 |                     333 |                    1 |
| json      |                              18 |                      55 |                    6 |

### Reproducing

The above was measured using Python 3.11.10 in a Fedora 42 container on an
x86-64-v4 machine using the
`orjson-3.10.11-cp311-cp311-manylinux_2_17_x86_64.manylinux2014_x86_64.whl`
artifact on PyPI. The latency results can be reproduced using the `pybench` script.

## Questions

### Why can't I install it from PyPI?

Probably `pip` needs to be upgraded to version 20.3 or later to support
the latest manylinux_x_y or universal2 wheel formats.

### "Cargo, the Rust package manager, is not installed or is not on PATH."

This happens when there are no binary wheels (like manylinux) for your
platform on PyPI. You can install [Rust](https://www.rust-lang.org/) through
`rustup` or a package manager and then it will compile.

### Will it deserialize to dataclasses, UUIDs, decimals, etc or support object_hook?

No. This requires a schema specifying what types are expected and how to
handle errors etc. This is addressed by data validation libraries a
level above this.

### Will it serialize to `str`?

No. `bytes` is the correct type for a serialized blob.

### Will it support NDJSON or JSONL?

No. [orjsonl](https://github.com/umarbutler/orjsonl) may be appropriate.

### Will it support JSON5 or RJSON?

No, it supports RFC 8259.

## Packaging

To package orjson requires at least [Rust](https://www.rust-lang.org/) 1.72
and the [maturin](https://github.com/PyO3/maturin) build tool. The recommended
build command is:

```sh
maturin build --release --strip
```

It benefits from also having a C build environment to compile a faster
deserialization backend. See this project's `manylinux_2_28` builds for an
example using clang and LTO.

The project's own CI tests against `nightly-2024-11-22` and stable 1.72. It
is prudent to pin the nightly version because that channel can introduce
breaking changes. There is a significant performance benefit to using
nightly.

orjson is tested for amd64, aarch64, and i686 on Linux and cross-compiles for
arm7, ppc64le, and s390x. It is tested for either aarch64 or amd64 on macOS and
cross-compiles for the other, depending on version. For Windows it is
tested on amd64 and i686.

There are no runtime dependencies other than libc.

The source distribution on PyPI contains all dependencies' source and can be
built without network access. The file can be downloaded from
`https://files.pythonhosted.org/packages/source/o/orjson/orjson-${version}.tar.gz`.

orjson's tests are included in the source distribution on PyPI. The
requirements to run the tests are specified in `test/requirements.txt`. The
tests should be run as part of the build. It can be run with
`pytest -q test`.

## License

orjson was written by ijl <<ijl@mailbox.org>>, copyright 2018 - 2024, available
to you under either the Apache 2 license or MIT license at your choice.
