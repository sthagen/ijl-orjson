// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2021-2026)

use crate::ffi::PyStrRef;

pub(crate) enum SerializeError {
    ArgsMissingPositional,
    ArgsMultipleDefault,
    ArgsMultipleOption,
    ArgsUnexpectedKeyword,
    ArgsInvalidOpts,
    DatetimeLibraryUnsupported,
    DefaultRecursionLimit,
    Integer53Bits,
    Integer64Bits,
    InvalidStr,
    InvalidFragment,
    KeyMustBeStr,
    RecursionLimit,
    TimeHasTzinfo,
    DictIntegerKey64Bit,
    DictKeyInvalidType,
    NumpyMalformed,
    NumpyNotCContiguous,
    NumpyNotNativeEndian,
    NumpyUnsupportedDatatype,
    NumpyUnsupportedDatetimeUnit(PyStrRef),
    UnsupportedType(PyStrRef),
}

impl SerializeError {
    pub const fn may_have_cause(&self) -> bool {
        match self {
            SerializeError::UnsupportedType(_) => true,
            SerializeError::ArgsMissingPositional
            | SerializeError::ArgsMultipleDefault
            | SerializeError::ArgsMultipleOption
            | SerializeError::ArgsUnexpectedKeyword
            | SerializeError::ArgsInvalidOpts
            | SerializeError::DatetimeLibraryUnsupported
            | SerializeError::DefaultRecursionLimit
            | SerializeError::Integer53Bits
            | SerializeError::Integer64Bits
            | SerializeError::InvalidStr
            | SerializeError::InvalidFragment
            | SerializeError::KeyMustBeStr
            | SerializeError::RecursionLimit
            | SerializeError::TimeHasTzinfo
            | SerializeError::DictIntegerKey64Bit
            | SerializeError::DictKeyInvalidType
            | SerializeError::NumpyMalformed
            | SerializeError::NumpyNotCContiguous
            | SerializeError::NumpyNotNativeEndian
            | SerializeError::NumpyUnsupportedDatatype
            | SerializeError::NumpyUnsupportedDatetimeUnit(_) => false,
        }
    }

    #[cold]
    #[inline(never)]
    pub fn to_pyunicode(&self) -> PyStrRef {
        match self {
            SerializeError::ArgsMissingPositional => {
                PyStrRef::from_str("dumps() missing 1 required positional argument: 'obj'")
            }
            SerializeError::ArgsMultipleDefault => {
                PyStrRef::from_str("dumps() got multiple values for argument: 'default'")
            }
            SerializeError::ArgsMultipleOption => {
                PyStrRef::from_str("dumps() got multiple values for argument: 'option'")
            }
            SerializeError::ArgsUnexpectedKeyword => {
                PyStrRef::from_str("dumps() got an unexpected keyword argument")
            }
            SerializeError::ArgsInvalidOpts => PyStrRef::from_str("Invalid opts"),
            SerializeError::DatetimeLibraryUnsupported => PyStrRef::from_str(
                "datetime's timezone library is not supported: use datetime.timezone.utc, pendulum, pytz, or dateutil",
            ),
            SerializeError::DefaultRecursionLimit => {
                PyStrRef::from_str("default serializer exceeds recursion limit")
            }
            SerializeError::Integer53Bits => PyStrRef::from_str("Integer exceeds 53-bit range"),
            SerializeError::Integer64Bits => PyStrRef::from_str("Integer exceeds 64-bit range"),
            SerializeError::InvalidStr => {
                PyStrRef::from_str("str is not valid UTF-8: surrogates not allowed")
            }
            SerializeError::InvalidFragment => {
                PyStrRef::from_str("orjson.Fragment's content is not of type bytes or str")
            }
            SerializeError::KeyMustBeStr => PyStrRef::from_str("Dict key must be str"),
            SerializeError::RecursionLimit => PyStrRef::from_str("Recursion limit reached"),
            SerializeError::TimeHasTzinfo => {
                PyStrRef::from_str("datetime.time must not have tzinfo set")
            }
            SerializeError::DictIntegerKey64Bit => {
                PyStrRef::from_str("Dict integer key must be within 64-bit range")
            }
            SerializeError::DictKeyInvalidType => {
                PyStrRef::from_str("Dict key must a type serializable with OPT_NON_STR_KEYS")
            }
            SerializeError::NumpyMalformed => PyStrRef::from_str("numpy array is malformed"),
            SerializeError::NumpyNotCContiguous => PyStrRef::from_str(
                "numpy array is not C contiguous; use ndarray.tolist() in default",
            ),
            SerializeError::NumpyNotNativeEndian => {
                PyStrRef::from_str("numpy array is not native-endianness")
            }
            SerializeError::NumpyUnsupportedDatatype => {
                PyStrRef::from_str("unsupported datatype in numpy array")
            }
            SerializeError::NumpyUnsupportedDatetimeUnit(msg) => *msg,
            SerializeError::UnsupportedType(msg) => *msg,
        }
    }
}
