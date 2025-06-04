use super::error::Result;
use core::fmt::Debug;
use vecno_grpc_core::{
    ops::VecnodPayloadOps,
    protowire::{VecnodRequest, VecnodResponse},
};
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;

pub(crate) mod id;
pub(crate) mod matcher;
pub(crate) mod queue;

pub(crate) trait Resolver: Send + Sync + Debug {
    fn register_request(&self, op: VecnodPayloadOps, request: &VecnodRequest) -> VecnodResponseReceiver;
    fn handle_response(&self, response: VecnodResponse);
    fn remove_expired_requests(&self, timeout: Duration);
}

pub(crate) type DynResolver = Arc<dyn Resolver>;

pub(crate) type VecnodResponseSender = oneshot::Sender<Result<VecnodResponse>>;
pub(crate) type VecnodResponseReceiver = oneshot::Receiver<Result<VecnodResponse>>;
