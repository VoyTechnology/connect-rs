use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Code {
    /// The zero code in gRPC is OK, which indicates that the operation was a
    /// success. We don't define a constant for it because it overlaps awkwardly
    /// with Go's error semantics: what does it mean to have a non-nil error with
    /// an OK status? (Also, the Connect protocol doesn't use a code for
    /// successes.)

    /// Canceled indicates that the operation was canceled, typically by the
    /// caller.
    Cancelled = 1,
    /// Unknown indicates that the operation failed for an unknown reason.
    Unknown,

    /// InvalidArgument indicates that client supplied an invalid argument.
    InvalidArgument,

    /// DeadlineExceeded indicates that deadline expired before the operation
    // could complete.
    DeadlineExceeded,

    /// NotFound indicates that some requested entity (for example, a file or
    /// directory) was not found.
    NotFound,

    /// AlreadyExists indicates that client attempted to create an entity (for
    /// example, a file or directory) that already exists.
    AlreadyExists,

    /// PermissionDenied indicates that the caller doesn't have permission to
    /// execute the specified operation.
    PermissionDenied,

    /// ResourceExhausted indicates that some resource has been exhausted. For
    /// example, a per-user quota may be exhausted or the entire file system may
    ///  be full.
    ResourceExhausted,

    /// FailedPrecondition indicates that the system is not in a state
    /// required for the operation's execution.
    FailedPrecondition,

    /// Aborted indicates that operation was aborted by the system, usually
    /// because of a concurrency issue such as a sequencer check failure or
    /// transaction abort.
    Aborted,

    /// OutOfRange indicates that the operation was attempted past the valid
    /// range (for example, seeking past end-of-file).
    OutOfRange,

    /// Unimplemented indicates that the operation isn't implemented,
    /// supported, or enabled in this service.
    Unimplemented,

    /// Internal indicates that some invariants expected by the underlying
    /// system have been broken. This code is reserved for serious errors.
    Internal,

    /// Unavailable indicates that the service is currently unavailable. This
    /// is usually temporary, so clients can back off and retry idempotent
    /// operations.
    Unavailable,

    /// DataLoss indicates that the operation has resulted in unrecoverable
    /// data loss or corruption.
    DataLoss,

    /// Unauthenticated indicates that the request does not have valid
    /// authentication credentials for the operation.
    Unauthenticated,
}

impl FromStr for Code {
    type Err = CodeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cancelled" => Ok(Self::Cancelled),
            "unknown" => Ok(Self::Unknown),
            "invalid_argument" => Ok(Self::InvalidArgument),
            "deadline_exceeded" => Ok(Self::DeadlineExceeded),
            "not_found" => Ok(Self::NotFound),
            "already_exists" => Ok(Self::AlreadyExists),
            "permission_denied" => Ok(Self::PermissionDenied),
            "resource_exhausted" => Ok(Self::ResourceExhausted),
            "failed_precondition" => Ok(Self::FailedPrecondition),
            "aborted" => Ok(Self::Aborted),
            "out_of_range" => Ok(Self::OutOfRange),
            "unimplemented" => Ok(Self::Unimplemented),
            "internal" => Ok(Self::Internal),
            "unavailable" => Ok(Self::Unavailable),
            "data_loss" => Ok(Self::DataLoss),
            "unauthenticated" => Ok(Self::Unauthenticated),
            _ => Err(CodeParseError),
        }
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Cancelled => "cancelled",
            Self::Unknown => "unknown",
            Self::InvalidArgument => "invalid_argument",
            Self::DeadlineExceeded => "deadline_exceeded",
            Self::NotFound => "not_found",
            Self::AlreadyExists => "already_exists",
            Self::PermissionDenied => "permission_denied",
            Self::ResourceExhausted => "resource_exhausted",
            Self::FailedPrecondition => "failed_precondition",
            Self::Aborted => "aborted",
            Self::OutOfRange => "out_of_range",
            Self::Unimplemented => "unimplemented",
            Self::Internal => "internal",
            Self::Unavailable => "unavailable",
            Self::DataLoss => "data_loss",
            Self::Unauthenticated => "unauthenticated",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct CodeParseError;

impl fmt::Display for CodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid code")
    }
}
