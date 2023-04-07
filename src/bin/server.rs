mod client;

use ockam::access_control::AllowAll;
use ockam::{Context, Error, Result, Routed, TcpInletTrustOptions, TcpListenerTrustOptions, TcpTransport, Worker};
use ockam::errcode::{Kind, Origin};
use ockam_transport_tcp::PortalMessage;
use ockam_test::{Command, Event};
use tracing::info;

#[derive(Default, Debug)]
pub struct ServerWorker;

#[ockam::worker]
impl Worker for ServerWorker {
    type Message = Event;
    type Context = Context;

    async fn handle_message(
        &mut self,
        context: &mut Self::Context,
        msg: Routed<Self::Message>,
    ) -> Result<()> {
        let route = msg.return_route();

        info!(
            "Address: {}, Received: {:?} from {}",
            context.address(),
            msg,
            route
        );

        let body = msg.body();

        match body {
            Event::Outbound(Command::RegisterClient) | Event::Inbound(Command::RegisterClient) => {
                info!("Received client registration: {:?}", body);
            }
            Event::Outbound(Command::RegisterUI) | Event::Inbound(Command::RegisterUI) => {
                info!("Received UI registration: {:?}", body);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Translator;

#[ockam::worker]
impl Worker for Translator {
    type Message = PortalMessage;
    type Context = Context;

    async fn handle_message(
        &mut self,
        context: &mut Self::Context,
        msg: Routed<Self::Message>,
    ) -> Result<()> {
        let return_route = msg.return_route();
        let body = msg.body();
        match body {
            PortalMessage::Ping => { context.send(return_route, PortalMessage::Pong).await }
            PortalMessage::Pong => { Ok(()) }
            PortalMessage::Disconnect => { self.shutdown(context).await }
            PortalMessage::Payload(body) => {
                let Ok(command) = serde_json::from_slice::<Event>(&body) else {
                    let result = match String::from_utf8(body) {
                        Ok(body_as_string) => {
                            Err(Error::new(Origin::Application, Kind::Invalid, format!("Failed to parse message as an Event: {}", body_as_string)))
                        }
                        // ignore non-string payloads
                        Err(_) => {
                            Ok(())
                        }
                    };
                    return result
                };
                context.send("server", command).await
            }
        }?;

        Ok(())
    }
}

#[ockam::node]
async fn main(context: &Context) -> Result<()> {
    let tcp = TcpTransport::create(&context).await?;

    tcp.listen("127.0.0.1:4000", TcpListenerTrustOptions::new())
        .await?;

    context
        .start_worker("server", ServerWorker, AllowAll, AllowAll)
        .await?;

    tcp.create_inlet("127.0.0.1:4001", "inlet", TcpInletTrustOptions::new())
        .await?;

    context
        .start_worker("inlet", Translator {}, AllowAll, AllowAll)
        .await?;

    Ok(())
}
