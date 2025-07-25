use std::sync::Arc;

use vecno_addressmanager::NetAddress;
use vecno_connectionmanager::ConnectionManager;
use vecno_core::{
    task::service::{AsyncService, AsyncServiceFuture},
    trace,
};
use vecno_p2p_lib::Adaptor;
use vecno_utils::triggers::SingleTrigger;
use vecno_utils_tower::counters::TowerConnectionCounters;

use crate::flow_context::FlowContext;

const P2P_CORE_SERVICE: &str = "p2p-service";

pub struct P2pService {
    flow_context: Arc<FlowContext>,
    connect_peers: Vec<NetAddress>,
    add_peers: Vec<NetAddress>,
    listen: NetAddress,
    outbound_target: usize,
    inbound_limit: usize,
    peers: &'static [&'static str],
    default_port: u16,
    shutdown: SingleTrigger,
    counters: Arc<TowerConnectionCounters>,
}

impl P2pService {
    pub fn new(
        flow_context: Arc<FlowContext>,
        connect_peers: Vec<NetAddress>,
        add_peers: Vec<NetAddress>,
        listen: NetAddress,
        outbound_target: usize,
        inbound_limit: usize,
        peers: &'static [&'static str],
        default_port: u16,
        counters: Arc<TowerConnectionCounters>,
    ) -> Self {
        Self {
            flow_context,
            connect_peers,
            add_peers,
            shutdown: SingleTrigger::default(),
            listen,
            outbound_target,
            inbound_limit,
            peers,
            default_port,
            counters,
        }
    }
}

impl AsyncService for P2pService {
    fn ident(self: Arc<Self>) -> &'static str {
        P2P_CORE_SERVICE
    }

    fn start(self: Arc<Self>) -> AsyncServiceFuture {
        trace!("{} starting", P2P_CORE_SERVICE);

        // Prepare a shutdown signal receiver
        let shutdown_signal = self.shutdown.listener.clone();

        let p2p_adaptor =
            Adaptor::bidirectional(self.listen, self.flow_context.hub().clone(), self.flow_context.clone(), self.counters.clone())
                .unwrap();
        let connection_manager = ConnectionManager::new(
            p2p_adaptor.clone(),
            self.outbound_target,
            self.inbound_limit,
            self.peers,
            self.default_port,
            self.flow_context.address_manager.clone(),
        );

        self.flow_context.set_connection_manager(connection_manager.clone());
        self.flow_context.start_async_services();

        // Launch the service and wait for a shutdown signal
        Box::pin(async move {
            for peer_address in self.connect_peers.iter().cloned().chain(self.add_peers.iter().cloned()) {
                connection_manager.add_connection_request(peer_address.into(), true).await;
            }

            // Keep the P2P server running until a service shutdown signal is received
            shutdown_signal.await;
            // Important for cleanup of the P2P adaptor since we have a reference cycle:
            // flow ctx -> conn manager -> p2p adaptor -> flow ctx (as ConnectionInitializer)
            self.flow_context.drop_connection_manager();
            p2p_adaptor.terminate_all_peers().await;
            connection_manager.stop().await;
            Ok(())
        })
    }

    fn signal_exit(self: Arc<Self>) {
        trace!("sending an exit signal to {}", P2P_CORE_SERVICE);
        self.shutdown.trigger.trigger();
    }

    fn stop(self: Arc<Self>) -> AsyncServiceFuture {
        Box::pin(async move {
            trace!("{} stopped", P2P_CORE_SERVICE);
            Ok(())
        })
    }
}
