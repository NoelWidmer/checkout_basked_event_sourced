pub enum SimulateError<HandleError> {
    CorruptionDetected, 
    CouldNotHandle(HandleError)
}
