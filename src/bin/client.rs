use ockam::access_control::AllowAll;
use ockam::{route, Context, Result, Routed, TcpConnectionTrustOptions, TcpTransport, Worker};

use ockam_test::{Command, Event};

#[derive(Debug)]
pub struct ClientWorker {
    route: ockam::Route,
}

#[ockam::worker]
impl Worker for ClientWorker {
    type Message = Event;
    type Context = Context;

    async fn handle_message(
        &mut self,
        context: &mut Self::Context,
        msg: Routed<Self::Message>,
    ) -> Result<()> {
        let route = msg.return_route();

        println!(
            "Address: {}, Received: {:?} from {}",
            context.address(),
            msg,
            route
        );

        Ok(())
    }

    async fn initialize(&mut self, context: &mut Self::Context) -> Result<()> {
        context
            .send(self.route.clone(), Event::Outbound(Command::RegisterClient))
            .await?;

        Ok(())
    }
}

#[ockam::node]
async fn main(context: &Context) -> Result<()> {
    let tcp = TcpTransport::create(&context).await?;

    let server_addr = tcp
        .connect("localhost:4000", TcpConnectionTrustOptions::new())
        .await?;
    let route = route![server_addr, "server"];

    context
        .start_worker("client", ClientWorker { route }, AllowAll, AllowAll)
        .await?;

    Ok(())
}
