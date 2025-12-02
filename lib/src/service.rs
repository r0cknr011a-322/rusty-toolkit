pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub trait Service<Request> {
    type Response;
    type Error;
    type Call: ServiceCall<Response = Self::Response, Error = Self::Error>;

    fn poll_ready(&mut self) -> Poll<Result<(), Self::Error>>;
    fn call(&mut self, req: Request) -> Self::Call;
}

pub trait ServiceCall {
    type Response;
    type Error;
    fn poll_call(&mut self) -> Poll<Result<Self::Response, Self::Error>>;
}
