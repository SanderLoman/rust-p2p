#![deny(unsafe_code)]

use discv5::{Discv5, Discv5Event};
use slog::Logger;

pub async fn discv5_events(discv5: &mut Discv5, log: Logger) {
    let mut event_stream = discv5.event_stream().await.unwrap();
    slog::info!(log, "Starting discv5 events");

    loop {
        match event_stream.recv().await.unwrap() {
            Discv5Event::Discovered(enr) => {
                slog::info!(log, "Discv5 event: Discovered"; "enr" => ?enr);
            }
            Discv5Event::EnrAdded { enr, replaced } => {
                slog::info!(log, "Discv5 event: EnrAdded"; "enr" => ?enr, "replaced" => ?replaced);
            }
            Discv5Event::NodeInserted { node_id, replaced } => {
                slog::info!(log, "Discv5 event: NodeInserted"; "node_id" => %node_id, "replaced" => ?replaced);
            }
            Discv5Event::SocketUpdated(socket_addr) => {
                slog::info!(log, "Discv5 event: SocketUpdated"; "socket_addr" => ?socket_addr);
            }
            Discv5Event::SessionEstablished(_, _) => {
                slog::info!(log, "Discv5 event: SessionEstablished");
            }
            Discv5Event::TalkRequest(_) => {
                slog::info!(log, "Discv5 event: TalkRequest");
            }
        }
    }
}
