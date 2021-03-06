extern crate tokio;
#[macro_use]
extern crate futures;
extern crate void;
#[macro_use]
extern crate log;
#[macro_use]
extern crate unwrap;
extern crate bytes;

use std::io;
use futures::{Future, Stream, Sink};

mod drop_notify;
mod until;
mod future_ext;
mod stream_ext;
mod first_ok;
mod log_errors;
mod log_error;
mod infallible;
mod next_or_else;
mod finally;
mod delay;
mod with_timeout;
mod thread_future;
mod first_ok2;
mod while_driving;
mod resume_unwind;
mod with_readiness_timeout;
pub mod bi_channel;
pub mod mpsc;
mod framed_unbuffered;

pub use drop_notify::{drop_notify, DropNotify, DropNotice};
pub use until::Until;
pub use first_ok::FirstOk;
pub use log_errors::LogErrors;
pub use log_error::LogError;
pub use future_ext::FutureExt;
pub use stream_ext::StreamExt;
pub use infallible::Infallible;
pub use next_or_else::NextOrElse;
pub use finally::Finally;
pub use with_timeout::WithTimeout;
pub use delay::Delay;
pub use with_readiness_timeout::WithReadinessTimeout;
pub use thread_future::{thread_future, ThreadFuture};
pub use first_ok2::FirstOk2;
pub use while_driving::{WhileDriving, Finish, FinishInner};
pub use resume_unwind::ResumeUnwind;
pub use framed_unbuffered::FramedUnbuffered;

pub type BoxFuture<T, E> = Box<Future<Item=T, Error=E>>;
pub type BoxStream<T, E> = Box<Stream<Item=T, Error=E>>;
pub type BoxSink<T, E> = Box<Sink<SinkItem=T, SinkError=E>>;

pub type BoxSendFuture<T, E> = Box<Future<Item=T, Error=E> + Send>;
pub type BoxSendStream<T, E> = Box<Stream<Item=T, Error=E> + Send>;
pub type BoxSendSink<T, E> = Box<Sink<SinkItem=T, SinkError=E> + Send>;

pub type IoFuture<T> = Box<Future<Item=T, Error=io::Error>>;
pub type IoStream<T> = Box<Stream<Item=T, Error=io::Error>>;
pub type IoSink<T> = Box<Sink<SinkItem=T, SinkError=io::Error>>;

