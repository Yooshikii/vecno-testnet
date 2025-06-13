use async_trait::async_trait;
use std::sync::Arc;
use vecno_consensus_core::config::Config;
use vecno_index_core::indexed_utxos::UtxoSetByScriptPublicKey;
use vecno_index_core::notification::{self as index_notify, Notification as IndexNotification};
use vecno_notify::converter::Converter;
use vecno_rpc_core::{utxo_set_into_rpc, Notification, RpcUtxosByAddressesEntry, UtxosChangedNotification};

/// Conversion of consensus_core to rpc_core structures
#[derive(Debug)]
pub struct IndexConverter {
    config: Arc<Config>,
}

impl IndexConverter {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn get_utxo_changed_notification(&self, utxo_changed: index_notify::UtxosChangedNotification) -> UtxosChangedNotification {
        UtxosChangedNotification {
            added: Arc::new(self.get_utxos_by_addresses_entries(&utxo_changed.added)),
            removed: Arc::new(self.get_utxos_by_addresses_entries(&utxo_changed.removed)),
        }
    }

    pub fn get_utxos_by_addresses_entries(&self, item: &UtxoSetByScriptPublicKey) -> Vec<RpcUtxosByAddressesEntry> {
        utxo_set_into_rpc(item, Some(self.config.prefix()))
    }
}

#[async_trait]
impl Converter for IndexConverter {
    type Incoming = IndexNotification;
    type Outgoing = Notification;

    async fn convert(&self, incoming: IndexNotification) -> Notification {
        match incoming {
            index_notify::Notification::UtxosChanged(msg) => Notification::UtxosChanged(self.get_utxo_changed_notification(msg)),
            _ => (&incoming).into(),
        }
    }
}
