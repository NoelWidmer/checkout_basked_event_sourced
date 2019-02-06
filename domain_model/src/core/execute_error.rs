pub enum ExecuteError<HandleError, ApplyError> {
    CorruptionDetected, 
    CouldNotHandle(HandleError), 
    CouldNotStore, 
    CouldNotApply(ApplyError)
}