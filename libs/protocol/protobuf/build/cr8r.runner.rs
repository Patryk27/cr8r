///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloReply {
    #[prost(string, tag = "1")]
    pub version: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRequest {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(string, tag = "2")]
    pub secret: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterReply {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportExperimentRequest {
    #[prost(string, tag = "1")]
    pub runner_id: std::string::String,
    #[prost(string, tag = "2")]
    pub experiment_id: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub report: ::std::option::Option<super::core::Report>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportExperimentReply {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestExperimentRequest {
    #[prost(string, tag = "1")]
    pub runner_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestExperimentReply {
    #[prost(message, optional, tag = "1")]
    pub assignment: ::std::option::Option<super::core::Assignment>,
}
#[doc = r" Generated client implementations."]
pub mod client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct RunnerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RunnerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RunnerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
        <T::ResponseBody as HttpBody>::Data: Into<bytes::Bytes> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        #[doc = r" Check if the service is ready."]
        pub async fn ready(&mut self) -> Result<(), tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })
        }
        pub async fn hello(
            &mut self,
            request: impl tonic::IntoRequest<super::HelloRequest>,
        ) -> Result<tonic::Response<super::HelloReply>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cr8r.runner.Runner/Hello");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn register(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterRequest>,
        ) -> Result<tonic::Response<super::RegisterReply>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cr8r.runner.Runner/Register");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn report_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportExperimentRequest>,
        ) -> Result<tonic::Response<super::ReportExperimentReply>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cr8r.runner.Runner/ReportExperiment");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn request_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestExperimentRequest>,
        ) -> Result<tonic::Response<super::RequestExperimentReply>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cr8r.runner.Runner/RequestExperiment");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for RunnerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RunnerServer."]
    #[async_trait]
    pub trait Runner: Send + Sync + 'static {
        async fn hello(
            &self,
            request: tonic::Request<super::HelloRequest>,
        ) -> Result<tonic::Response<super::HelloReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn register(
            &self,
            request: tonic::Request<super::RegisterRequest>,
        ) -> Result<tonic::Response<super::RegisterReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn report_experiment(
            &self,
            request: tonic::Request<super::ReportExperimentRequest>,
        ) -> Result<tonic::Response<super::ReportExperimentReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn request_experiment(
            &self,
            request: tonic::Request<super::RequestExperimentRequest>,
        ) -> Result<tonic::Response<super::RequestExperimentReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct RunnerServer<T: Runner> {
        inner: Arc<T>,
    }
    impl<T: Runner> RunnerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            Self { inner }
        }
    }
    impl<T: Runner> Service<http::Request<HyperBody>> for RunnerServer<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/cr8r.runner.Runner/Hello" => {
                    struct HelloSvc<T: Runner>(pub Arc<T>);
                    impl<T: Runner> tonic::server::UnaryService<super::HelloRequest> for HelloSvc<T> {
                        type Response = super::HelloReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HelloRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.hello(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = HelloSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cr8r.runner.Runner/Register" => {
                    struct RegisterSvc<T: Runner>(pub Arc<T>);
                    impl<T: Runner> tonic::server::UnaryService<super::RegisterRequest> for RegisterSvc<T> {
                        type Response = super::RegisterReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.register(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RegisterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cr8r.runner.Runner/ReportExperiment" => {
                    struct ReportExperimentSvc<T: Runner>(pub Arc<T>);
                    impl<T: Runner> tonic::server::UnaryService<super::ReportExperimentRequest>
                        for ReportExperimentSvc<T>
                    {
                        type Response = super::ReportExperimentReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReportExperimentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.report_experiment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ReportExperimentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cr8r.runner.Runner/RequestExperiment" => {
                    struct RequestExperimentSvc<T: Runner>(pub Arc<T>);
                    impl<T: Runner> tonic::server::UnaryService<super::RequestExperimentRequest>
                        for RequestExperimentSvc<T>
                    {
                        type Response = super::RequestExperimentReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestExperimentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.request_experiment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RequestExperimentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Runner> Clone for RunnerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Runner> tonic::transport::ServiceName for RunnerServer<T> {
        const NAME: &'static str = "cr8r.runner.Runner";
    }
}
