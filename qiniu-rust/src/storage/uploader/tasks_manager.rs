use qiniu_http::Error as HTTPError;
use std::{
    io::{Error as IOError, ErrorKind as IOErrorKind, Read},
    marker::PhantomData,
    sync::Mutex,
};

pub(super) enum Result {
    Success,
    IOError(IOError),
    HTTPError(HTTPError),
}

enum Status<'r, R: Read + Send + Sync + 'r> {
    Uploading(R, PhantomData<&'r R>),
    IOError(IOError),
    HTTPError(HTTPError),
    Success,
}

pub(super) enum Task {
    Upload(usize),
    End,
}

pub(super) struct TasksManager<'r, R: Read + Send + Sync + 'r> {
    inner: Mutex<Status<'r, R>>,
}

impl<'r, R: Read + Send + Sync + 'r> TasksManager<'r, R> {
    pub(super) fn new(io: R) -> TasksManager<'r, R> {
        TasksManager {
            inner: Mutex::new(Status::Uploading(io, PhantomData)),
        }
    }

    pub(super) fn get_task(&self, buf: &mut [u8]) -> Task {
        let mut lock = self.inner.lock().unwrap();
        match &mut *lock {
            Status::Uploading(io, _) => {
                let mut have_read = 0;
                loop {
                    match io.read(&mut buf[have_read..]) {
                        Ok(0) => {
                            *lock = Status::Success;
                            if have_read > 0 {
                                return Task::Upload(have_read);
                            } else {
                                return Task::End;
                            }
                        }
                        Ok(n) => {
                            have_read += n;
                            if have_read == buf.len() {
                                return Task::Upload(have_read);
                            }
                        }
                        Err(ref err) if err.kind() == IOErrorKind::Interrupted => {
                            continue;
                        }
                        Err(err) => {
                            *lock = Status::IOError(err);
                            return Task::End;
                        }
                    }
                }
            }
            _ => Task::End,
        }
    }

    pub(super) fn error(&self, err: HTTPError) {
        *self.inner.lock().unwrap() = Status::HTTPError(err);
    }

    pub(super) fn result(self) -> Result {
        match self.inner.into_inner().unwrap() {
            Status::Success => Result::Success,
            Status::IOError(err) => Result::IOError(err),
            Status::HTTPError(err) => Result::HTTPError(err),
            Status::Uploading(_, _) => {
                panic!("Unexpected uploading status of task_manager");
            }
        }
    }
}