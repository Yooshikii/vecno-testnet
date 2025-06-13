use crate::{flow_context::FlowContext, flow_trait::Flow};
use std::sync::Arc;
use vecno_core::debug;
use vecno_p2p_lib::{
    common::ProtocolError,
    dequeue_with_request_id, make_message, make_response,
    pb::{vecnod_message::Payload, InvRelayBlockMessage},
    IncomingRoute, Router,
};

pub struct HandleRelayBlockRequests {
    ctx: FlowContext,
    router: Arc<Router>,
    incoming_route: IncomingRoute,
}

#[async_trait::async_trait]
impl Flow for HandleRelayBlockRequests {
    fn router(&self) -> Option<Arc<Router>> {
        Some(self.router.clone())
    }

    async fn start(&mut self) -> Result<(), ProtocolError> {
        self.start_impl().await
    }
}

impl HandleRelayBlockRequests {
    pub fn new(ctx: FlowContext, router: Arc<Router>, incoming_route: IncomingRoute) -> Self {
        Self { ctx, router, incoming_route }
    }

    async fn start_impl(&mut self) -> Result<(), ProtocolError> {
        self.send_sink().await?;
        loop {
            let (msg, request_id) = dequeue_with_request_id!(self.incoming_route, Payload::RequestRelayBlocks)?;
            let hashes: Vec<_> = msg.try_into()?;

            let session = self.ctx.consensus().unguarded_session();

            for hash in hashes {
                let block = session.async_get_block(hash).await?;
                self.router.enqueue(make_response!(Payload::Block, (&block).into(), request_id)).await?;
                debug!("relayed block with hash {} to peer {}", hash, self.router);
            }
        }
    }

    async fn send_sink(&mut self) -> Result<(), ProtocolError> {
        let sink = self.ctx.consensus().unguarded_session().async_get_sink().await;
        if sink == self.ctx.config.genesis.hash {
            return Ok(());
        }
        self.router.enqueue(make_message!(Payload::InvRelayBlock, InvRelayBlockMessage { hash: Some(sink.into()) })).await?;
        Ok(())
    }
}
