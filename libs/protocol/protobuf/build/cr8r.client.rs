///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloReply {
    #[prost(string, tag = "1")]
    pub version: std::string::String,
    #[prost(uint64, tag = "2")]
    pub uptime: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbortExperimentRequest {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbortExperimentReply {}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindExperimentsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindExperimentsReply {
    #[prost(message, repeated, tag = "1")]
    pub experiments: ::std::vec::Vec<super::core::Experiment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaunchExperimentRequest {
    #[prost(message, optional, tag = "1")]
    pub experiment: ::std::option::Option<super::core::ExperimentDefinition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaunchExperimentReply {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    #[prost(uint32, tag = "2")]
    pub position_in_queue: u32,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindRunnersRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindRunnersReply {
    #[prost(message, repeated, tag = "1")]
    pub runners: ::std::vec::Vec<super::core::Runner>,
}
#[doc = r" Generated client implementations."]
pub mod client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct ClientClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ClientClient<tonic::transport::Channel> {
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
    impl<T> ClientClient<T>
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
            let path = http::uri::PathAndQuery::from_static("/cr8r.client.Client/Hello");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn abort_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::AbortExperimentRequest>,
        ) -> Result<tonic::Response<super::AbortExperimentReply>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cr8r.client.Client/AbortExperiment");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn find_experiments(
            &mut self,
            request: impl tonic::IntoRequest<super::FindExperimentsRequest>,
        ) -> Result<tonic::Response<super::FindExperimentsReply>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cr8r.client.Client/FindExperiments");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn launch_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::LaunchExperimentRequest>,
        ) -> Result<tonic::Response<super::LaunchExperimentReply>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cr8r.client.Client/LaunchExperiment");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn find_runners(
            &mut self,
            request: impl tonic::IntoRequest<super::FindRunnersRequest>,
        ) -> Result<tonic::Response<super::FindRunnersReply>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cr8r.client.Client/FindRunners");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ClientClient<T> {
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
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ClientServer."]
    #[async_trait]
    pub trait Client: Send + Sync + 'static {
        async fn hello(
            &self,
            request: tonic::Request<super::HelloRequest>,
        ) -> Result<tonic::Response<super::HelloReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn abort_experiment(
            &self,
            request: tonic::Request<super::AbortExperimentRequest>,
        ) -> Result<tonic::Response<super::AbortExperimentReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn find_experiments(
            &self,
            request: tonic::Request<super::FindExperimentsRequest>,
        ) -> Result<tonic::Response<super::FindExperimentsReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn launch_experiment(
            &self,
            request: tonic::Request<super::LaunchExperimentRequest>,
        ) -> Result<tonic::Response<super::LaunchExperimentReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn find_runners(
            &self,
            request: tonic::Request<super::FindRunnersRequest>,
        ) -> Result<tonic::Response<super::FindRunnersReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ClientServer<T: Client> {
        inner: Arc<T>,
    }
    impl<T: Client> ClientServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            Self { inner }
        }
    }
    impl<T: Client> Service<http::Request<HyperBody>> for ClientServer<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/cr8r.client.Client/Hello" => {
                    struct HelloSvc<T: Client>(pub Arc<T>);
                    impl<T: Client> tonic::server::UnaryService<super::HelloRequest> for HelloSvc<T> {
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
                "/cr8r.client.Client/AbortExperiment" => {
                    struct AbortExperimentSvc<T: Client>(pub Arc<T>);
                    impl<T: Client> tonic::server::UnaryService<super::AbortExperimentRequest>
                        for AbortExperimentSvc<T>
                    {
                        type Response = super::AbortExperimentReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AbortExperimentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.abort_experiment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AbortExperimentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cr8r.client.Client/FindExperiments" => {
                    struct FindExperimentsSvc<T: Client>(pub Arc<T>);
                    impl<T: Client> tonic::server::UnaryService<super::FindExperimentsRequest>
                        for FindExperimentsSvc<T>
                    {
                        type Response = super::FindExperimentsReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FindExperimentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.find_experiments(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = FindExperimentsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cr8r.client.Client/LaunchExperiment" => {
                    struct LaunchExperimentSvc<T: Client>(pub Arc<T>);
                    impl<T: Client> tonic::server::UnaryService<super::LaunchExperimentRequest>
                        for LaunchExperimentSvc<T>
                    {
                        type Response = super::LaunchExperimentReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LaunchExperimentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.launch_experiment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = LaunchExperimentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cr8r.client.Client/FindRunners" => {
                    struct FindRunnersSvc<T: Client>(pub Arc<T>);
                    impl<T: Client> tonic::server::UnaryService<super::FindRunnersRequest> for FindRunnersSvc<T> {
                        type Response = super::FindRunnersReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FindRunnersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.find_runners(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = FindRunnersSvc(inner);
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
    impl<T: Client> Clone for ClientServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Client> tonic::transport::ServiceName for ClientServer<T> {
        const NAME: &'static str = "cr8r.client.Client";
    }
}
