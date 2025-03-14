use super::method::{DropFn, Method, MethodTrait, RoutingPolicy};
use crate::{
    connection::Connection,
    connection_handler::ServerContext,
    error::{GrpcServerError, GrpcServerResult},
};
use entropyx_grpc_core::{
    ops::EntropyXPayloadOps,
    protowire::{EntropyXRequest, EntropyXResponse},
};
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};

pub type EntropyXMethod = Method<ServerContext, Connection, EntropyXRequest, EntropyXResponse>;
pub type DynEntropyXMethod = Arc<dyn MethodTrait<ServerContext, Connection, EntropyXRequest, EntropyXResponse>>;
pub type EntropyXDropFn = DropFn<EntropyXRequest, EntropyXResponse>;
pub type EntropyXRoutingPolicy = RoutingPolicy<EntropyXRequest, EntropyXResponse>;

/// An interface providing methods implementations and a fallback "not implemented" method
/// actually returning a message with a "not implemented" error.
///
/// The interface can provide a method clone for every [`EntropyXPayloadOps`] variant for later
/// processing of related requests.
///
/// It is also possible to directly let the interface itself process a request by invoking
/// the `call()` method.
pub struct Interface {
    server_ctx: ServerContext,
    methods: HashMap<EntropyXPayloadOps, DynEntropyXMethod>,
    method_not_implemented: DynEntropyXMethod,
}

impl Interface {
    pub fn new(server_ctx: ServerContext) -> Self {
        let method_not_implemented = Arc::new(Method::new(|_, _, entropy_x_request: EntropyXRequest| {
            Box::pin(async move {
                match entropy_x_request.payload {
                    Some(ref request) => Ok(EntropyXResponse {
                        id: entropy_x_request.id,
                        payload: Some(EntropyXPayloadOps::from(request).to_error_response(GrpcServerError::MethodNotImplemented.into())),
                    }),
                    None => Err(GrpcServerError::InvalidRequestPayload),
                }
            })
        }));
        Self { server_ctx, methods: Default::default(), method_not_implemented }
    }

    pub fn method(&mut self, op: EntropyXPayloadOps, method: EntropyXMethod) {
        let method: DynEntropyXMethod = Arc::new(method);
        if self.methods.insert(op, method).is_some() {
            panic!("RPC method {op:?} is declared multiple times")
        }
    }

    pub fn replace_method(&mut self, op: EntropyXPayloadOps, method: EntropyXMethod) {
        let method: DynEntropyXMethod = Arc::new(method);
        let _ = self.methods.insert(op, method);
    }

    pub fn set_method_properties(
        &mut self,
        op: EntropyXPayloadOps,
        tasks: usize,
        queue_size: usize,
        routing_policy: EntropyXRoutingPolicy,
    ) {
        self.methods.entry(op).and_modify(|x| {
            let method: Method<ServerContext, Connection, EntropyXRequest, EntropyXResponse> =
                Method::with_properties(x.method_fn(), tasks, queue_size, routing_policy);
            let method: Arc<dyn MethodTrait<ServerContext, Connection, EntropyXRequest, EntropyXResponse>> = Arc::new(method);
            *x = method;
        });
    }

    pub async fn call(
        &self,
        op: &EntropyXPayloadOps,
        connection: Connection,
        request: EntropyXRequest,
    ) -> GrpcServerResult<EntropyXResponse> {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).call(self.server_ctx.clone(), connection, request).await
    }

    pub fn get_method(&self, op: &EntropyXPayloadOps) -> DynEntropyXMethod {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).clone()
    }
}

impl Debug for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interface").finish()
    }
}
