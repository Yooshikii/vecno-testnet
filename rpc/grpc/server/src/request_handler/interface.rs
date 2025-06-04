use super::method::{DropFn, Method, MethodTrait, RoutingPolicy};
use crate::{
    connection::Connection,
    connection_handler::ServerContext,
    error::{GrpcServerError, GrpcServerResult},
};
use vecno_grpc_core::{
    ops::VecnodPayloadOps,
    protowire::{VecnodRequest, VecnodResponse},
};
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};

pub type VecnodMethod = Method<ServerContext, Connection, VecnodRequest, VecnodResponse>;
pub type DynVecnodMethod = Arc<dyn MethodTrait<ServerContext, Connection, VecnodRequest, VecnodResponse>>;
pub type VecnodDropFn = DropFn<VecnodRequest, VecnodResponse>;
pub type VecnodRoutingPolicy = RoutingPolicy<VecnodRequest, VecnodResponse>;

/// An interface providing methods implementations and a fallback "not implemented" method
/// actually returning a message with a "not implemented" error.
///
/// The interface can provide a method clone for every [`VecnodPayloadOps`] variant for later
/// processing of related requests.
///
/// It is also possible to directly let the interface itself process a request by invoking
/// the `call()` method.
pub struct Interface {
    server_ctx: ServerContext,
    methods: HashMap<VecnodPayloadOps, DynVecnodMethod>,
    method_not_implemented: DynVecnodMethod,
}

impl Interface {
    pub fn new(server_ctx: ServerContext) -> Self {
        let method_not_implemented = Arc::new(Method::new(|_, _, vecnod_request: VecnodRequest| {
            Box::pin(async move {
                match vecnod_request.payload {
                    Some(ref request) => Ok(VecnodResponse {
                        id: vecnod_request.id,
                        payload: Some(VecnodPayloadOps::from(request).to_error_response(GrpcServerError::MethodNotImplemented.into())),
                    }),
                    None => Err(GrpcServerError::InvalidRequestPayload),
                }
            })
        }));
        Self { server_ctx, methods: Default::default(), method_not_implemented }
    }

    pub fn method(&mut self, op: VecnodPayloadOps, method: VecnodMethod) {
        let method: DynVecnodMethod = Arc::new(method);
        if self.methods.insert(op, method).is_some() {
            panic!("RPC method {op:?} is declared multiple times")
        }
    }

    pub fn replace_method(&mut self, op: VecnodPayloadOps, method: VecnodMethod) {
        let method: DynVecnodMethod = Arc::new(method);
        let _ = self.methods.insert(op, method);
    }

    pub fn set_method_properties(
        &mut self,
        op: VecnodPayloadOps,
        tasks: usize,
        queue_size: usize,
        routing_policy: VecnodRoutingPolicy,
    ) {
        self.methods.entry(op).and_modify(|x| {
            let method: Method<ServerContext, Connection, VecnodRequest, VecnodResponse> =
                Method::with_properties(x.method_fn(), tasks, queue_size, routing_policy);
            let method: Arc<dyn MethodTrait<ServerContext, Connection, VecnodRequest, VecnodResponse>> = Arc::new(method);
            *x = method;
        });
    }

    pub async fn call(
        &self,
        op: &VecnodPayloadOps,
        connection: Connection,
        request: VecnodRequest,
    ) -> GrpcServerResult<VecnodResponse> {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).call(self.server_ctx.clone(), connection, request).await
    }

    pub fn get_method(&self, op: &VecnodPayloadOps) -> DynVecnodMethod {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).clone()
    }
}

impl Debug for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interface").finish()
    }
}
