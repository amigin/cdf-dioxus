use dioxus::prelude::*;
use my_nosql_contracts::BidAskSnapshotNoSqlEntity;
use rust_extensions::lazy::LazyVec;

use crate::{
    states::{BidAskSnapshotState, InstrumentsState},
    types::*,
    APP_CTX,
};

#[inline_props]
pub fn render_rate(cx: &Scope, instrument_id: InstrumentId) -> Element {
    let bid_ask_snapshot = use_shared_state::<BidAskSnapshotState>(cx).unwrap();

    if !bid_ask_snapshot.read().thread_is_taken {
        bid_ask_snapshot.write().thread_is_taken = true;

        let bid_ask_snapshot_state = bid_ask_snapshot.to_owned();

        cx.spawn(async move {
            bid_ask_snapshot_state.write().thread_is_taken = true;
            let readers = APP_CTX.get_my_no_sql_readers().await;

            loop {
                let readers = readers.clone();

                let result = tokio::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_micros(300)).await;
                    readers
                        .bid_ask
                        .get_by_partition_key_as_vec(
                            BidAskSnapshotNoSqlEntity::generate_partition_key(),
                        )
                        .await
                })
                .await
                .unwrap();

                let my_snapshot = bid_ask_snapshot_state.read().bid_ask.clone();

                if let Some(result) = result {
                    let mut to_update = LazyVec::with_capacity(result.len());

                    for itm in result {
                        let bid_ask: BidAsk = itm.as_ref().into();

                        let has_to_be_updated = if let Some(my_bid_ask) =
                            my_snapshot.get(bid_ask.instrument_id.as_str())
                        {
                            if bid_ask.is_same_with(my_bid_ask) {
                                None
                            } else {
                                Some(bid_ask)
                            }
                        } else {
                            Some(bid_ask)
                        };

                        if let Some(has_to_be_updated) = has_to_be_updated {
                            to_update.add(has_to_be_updated.clone());
                        };
                    }

                    if let Some(to_update) = to_update.get_result() {
                        bid_ask_snapshot_state.write().update(to_update);
                    }
                }
            }
        });
    }

    let instruments = use_shared_state::<InstrumentsState>(cx).unwrap();

    let instrument = instruments.read();
    let instrument = instrument.get(&instrument_id);

    let rate = bid_ask_snapshot
        .read()
        .get_rate_as_str(instrument, &instrument_id);

    render! { span { "{rate}" } }
}
