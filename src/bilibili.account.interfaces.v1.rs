// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MidByNameReply {
    ///
    #[prost(map = "string, int64", tag = "1")]
    pub name_map: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MidByNameReq {
    ///
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnameMessageReply {
    ///
    #[prost(bool, tag = "1")]
    pub allow: bool,
    ///
    #[prost(bool, tag = "2")]
    pub realname_or_tel: bool,
    ///
    #[prost(string, tag = "3")]
    pub uname_message: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub confirm_message: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub condition_message: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub bind_tel: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct UnameMessageReq {}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNameReply {
    ///
    #[prost(int64, tag = "1")]
    pub code: i64,
    ///
    #[prost(string, tag = "2")]
    pub msg: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "3")]
    pub name_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNameReq {
    ///
    #[prost(string, tag = "1")]
    pub uname: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod account_interface_v1_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct AccountInterfaceV1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AccountInterfaceV1Client<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AccountInterfaceV1Client<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            AccountInterfaceV1Client::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        ///
        pub async fn mid_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::MidByNameReq>,
        ) -> std::result::Result<tonic::Response<super::MidByNameReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.account.interfaces.v1.AccountInterfaceV1/MidByName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.account.interfaces.v1.AccountInterfaceV1",
                        "MidByName",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn uname_message(
            &mut self,
            request: impl tonic::IntoRequest<super::UnameMessageReq>,
        ) -> std::result::Result<
            tonic::Response<super::UnameMessageReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.account.interfaces.v1.AccountInterfaceV1/UnameMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.account.interfaces.v1.AccountInterfaceV1",
                        "UnameMessage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn update_name_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNameReq>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateNameReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.account.interfaces.v1.AccountInterfaceV1/UpdateNameV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.account.interfaces.v1.AccountInterfaceV1",
                        "UpdateNameV2",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod account_interface_v1_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AccountInterfaceV1Server.
    #[async_trait]
    pub trait AccountInterfaceV1: std::marker::Send + std::marker::Sync + 'static {
        ///
        async fn mid_by_name(
            &self,
            request: tonic::Request<super::MidByNameReq>,
        ) -> std::result::Result<tonic::Response<super::MidByNameReply>, tonic::Status>;
        ///
        async fn uname_message(
            &self,
            request: tonic::Request<super::UnameMessageReq>,
        ) -> std::result::Result<
            tonic::Response<super::UnameMessageReply>,
            tonic::Status,
        >;
        ///
        async fn update_name_v2(
            &self,
            request: tonic::Request<super::UpdateNameReq>,
        ) -> std::result::Result<tonic::Response<super::UpdateNameReply>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct AccountInterfaceV1Server<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> AccountInterfaceV1Server<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AccountInterfaceV1Server<T>
    where
        T: AccountInterfaceV1,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/bilibili.account.interfaces.v1.AccountInterfaceV1/MidByName" => {
                    #[allow(non_camel_case_types)]
                    struct MidByNameSvc<T: AccountInterfaceV1>(pub Arc<T>);
                    impl<
                        T: AccountInterfaceV1,
                    > tonic::server::UnaryService<super::MidByNameReq>
                    for MidByNameSvc<T> {
                        type Response = super::MidByNameReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MidByNameReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountInterfaceV1>::mid_by_name(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = MidByNameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bilibili.account.interfaces.v1.AccountInterfaceV1/UnameMessage" => {
                    #[allow(non_camel_case_types)]
                    struct UnameMessageSvc<T: AccountInterfaceV1>(pub Arc<T>);
                    impl<
                        T: AccountInterfaceV1,
                    > tonic::server::UnaryService<super::UnameMessageReq>
                    for UnameMessageSvc<T> {
                        type Response = super::UnameMessageReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnameMessageReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountInterfaceV1>::uname_message(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UnameMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bilibili.account.interfaces.v1.AccountInterfaceV1/UpdateNameV2" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateNameV2Svc<T: AccountInterfaceV1>(pub Arc<T>);
                    impl<
                        T: AccountInterfaceV1,
                    > tonic::server::UnaryService<super::UpdateNameReq>
                    for UpdateNameV2Svc<T> {
                        type Response = super::UpdateNameReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateNameReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountInterfaceV1>::update_name_v2(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateNameV2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(empty_body());
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for AccountInterfaceV1Server<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "bilibili.account.interfaces.v1.AccountInterfaceV1";
    impl<T> tonic::server::NamedService for AccountInterfaceV1Server<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}