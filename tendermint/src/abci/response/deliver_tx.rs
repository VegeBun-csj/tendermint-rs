use bytes::Bytes;
use serde::{Deserialize, Serialize};

use super::super::{Code, Event};
use crate::prelude::*;
use crate::serializers;

#[doc = include_str!("../doc/response-delivertx.md")]
#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DeliverTx {
    /// The response code.
    ///
    /// This code should be `0` only if the transaction is fully valid. However,
    /// invalid transactions included in a block will still be executed against
    /// the application state.
    pub code: Code,
    /// Result bytes, if any.
    #[serde(with = "serializers::nullable")]
    pub data: Bytes,
    /// The output of the application's logger.
    ///
    /// **May be non-deterministic**.
    pub log: String,
    /// Additional information.
    ///
    /// **May be non-deterministic**.
    pub info: String,
    /// Amount of gas requested for the transaction.
    #[serde(with = "serializers::from_str")]
    pub gas_wanted: i64,
    /// Amount of gas consumed by the transaction.
    #[serde(with = "serializers::from_str")]
    pub gas_used: i64,
    /// Events that occurred while executing the transaction.
    pub events: Vec<Event>,
    /// The namespace for the `code`.
    pub codespace: String,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

use core::convert::{TryFrom, TryInto};

use tendermint_proto::{abci as pb, Protobuf};

impl From<DeliverTx> for pb::ResponseDeliverTx {
    fn from(deliver_tx: DeliverTx) -> Self {
        Self {
            code: deliver_tx.code.into(),
            data: deliver_tx.data,
            log: deliver_tx.log,
            info: deliver_tx.info,
            gas_wanted: deliver_tx.gas_wanted,
            gas_used: deliver_tx.gas_used,
            events: deliver_tx.events.into_iter().map(Into::into).collect(),
            codespace: deliver_tx.codespace,
        }
    }
}

impl TryFrom<pb::ResponseDeliverTx> for DeliverTx {
    type Error = crate::Error;

    fn try_from(deliver_tx: pb::ResponseDeliverTx) -> Result<Self, Self::Error> {
        Ok(Self {
            code: deliver_tx.code.into(),
            data: deliver_tx.data,
            log: deliver_tx.log,
            info: deliver_tx.info,
            gas_wanted: deliver_tx.gas_wanted,
            gas_used: deliver_tx.gas_used,
            events: deliver_tx
                .events
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            codespace: deliver_tx.codespace,
        })
    }
}

impl Protobuf<pb::ResponseDeliverTx> for DeliverTx {}
