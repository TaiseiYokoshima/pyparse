pub enum TokenizerError<'a> {
    FileNotFound(&'a String),
    PermissionDenied,
    Other(&'a String),
}
