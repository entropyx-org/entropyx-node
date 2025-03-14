use super::error::Result;
use core::fmt::Debug;
use entropyx_grpc_core::{
    ops::EntropyXPayloadOps,
    protowire::{EntropyXRequest, EntropyXResponse},
};
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;

pub(crate) mod id;
pub(crate) mod matcher;
pub(crate) mod queue;

pub(crate) trait Resolver: Send + Sync + Debug {
    fn register_request(&self, op: EntropyXPayloadOps, request: &EntropyXRequest) -> EntropyXResponseReceiver;
    fn handle_response(&self, response: EntropyXResponse);
    fn remove_expired_requests(&self, timeout: Duration);
}

pub(crate) type DynResolver = Arc<dyn Resolver>;

pub(crate) type EntropyXResponseSender = oneshot::Sender<Result<EntropyXResponse>>;
pub(crate) type EntropyXResponseReceiver = oneshot::Receiver<Result<EntropyXResponse>>;
