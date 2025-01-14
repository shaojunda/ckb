use crate::{Error, ErrorKind};
use failure::{err_msg, Backtrace, Context, Fail};
use std::fmt::{self, Debug, Display};

#[derive(Debug)]
pub struct InternalError {
    kind: Context<InternalErrorKind>,
}

#[derive(Debug, PartialEq, Eq, Clone, Display)]
pub enum InternalErrorKind {
    /// An arithmetic overflow occurs during capacity calculation,
    /// e.g. `Capacity::safe_add`
    CapacityOverflow,

    /// The transaction_pool is already full
    TransactionPoolFull,

    /// The transaction already exist in transaction_pool
    PoolTransactionDuplicated,

    /// Persistent data had corrupted
    DataCorrupted,

    /// Database exception
    Database,

    /// VM internal error
    VM,

    /// Unknown system error
    System,
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(cause) = self.cause() {
            write!(f, "{}({})", self.kind(), cause)
        } else {
            write!(f, "{}", self.kind())
        }
    }
}

impl From<InternalError> for Error {
    fn from(error: InternalError) -> Self {
        error.context(ErrorKind::Internal).into()
    }
}

impl From<InternalErrorKind> for InternalError {
    fn from(kind: InternalErrorKind) -> Self {
        InternalError {
            kind: Context::new(kind),
        }
    }
}

impl From<InternalErrorKind> for Error {
    fn from(kind: InternalErrorKind) -> Self {
        Into::<InternalError>::into(kind).into()
    }
}

impl InternalErrorKind {
    pub fn cause<F: Fail>(self, cause: F) -> InternalError {
        InternalError {
            kind: cause.context(self),
        }
    }

    pub fn reason<S: Display + Debug + Sync + Send + 'static>(self, reason: S) -> InternalError {
        InternalError {
            kind: err_msg(reason).compat().context(self),
        }
    }
}

impl InternalError {
    pub fn kind(&self) -> &InternalErrorKind {
        &self.kind.get_context()
    }
}

impl Fail for InternalError {
    fn cause(&self) -> Option<&Fail> {
        self.kind.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.kind.backtrace()
    }
}
