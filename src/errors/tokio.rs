


use tokio::time::error::Error as TimeError;
use tokio::io::Error as IoError;
use tokio::io::ErrorKind as ErrorKind;
use tokio::sync::{SetError, TryLockError, TryAcquireError, AcquireError};
use tokio::task::JoinError;
use tokio::sync::oneshot::error::{RecvError, TryRecvError};
use tokio::sync::watch::error::{SendError, RecvError as WatchRecvError};
use tokio::sync::broadcast::error::{RecvError, TryRecvError, SendError};
use tokio::sync::{TryAcquireError, SetError, AcquireError, TryLockError};   //Try these ones, check if theya re actually errors
use tokio::net::tcp::ReuniteError;
use tokio::sync::mpsc::error::{SendError, TryRecvError, SendTimeoutError, TrySendError};
use tokio::runtime::TryCurrentError;