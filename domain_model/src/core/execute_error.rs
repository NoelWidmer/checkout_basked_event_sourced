pub enum ExecuteError<HandleError, ApplyError> {
    CorruptionDetected, 
    CouldNotHandle(HandleError), 
    CouldNotApply(ApplyError)
}