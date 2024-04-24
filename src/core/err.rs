use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum InitError {
    #[error("OpenInit is already running")]
    AlreadyRunning,

    #[error("OpenInit could not create the pid file")]
    PidError,
    
    #[error("OpenInit could not read the service directory")]
    ServiceDirectoryNotFound,
}