mod crd;

use std::sync::Arc;
use futures::StreamExt;
use kube::{
    runtime::{
        controller::{self, Controller},
        watcher,
    },
    Api, Client,
};
use tracing::*;
use crd::OfferDeployment;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let client = Client::try_default().await?;
    let api: Api<OfferDeployment> = Api::all(client.clone());

    info!("Starting offer-operator... watching OfferDeployments");

    Controller::new(api, watcher::Config::default())
        .run(reconcile, error_policy, Arc::new(()))
        .for_each(|res| async move {
            match res {
                Ok(action) => info!("Reconciliation complete: {:?}", action),
                Err(err) => error!("Reconcile failed: {:?}", err),
            }
        })
        .await;

    Ok(())
}

async fn reconcile(
    _obj: Arc<OfferDeployment>,
    _ctx: Arc<()>,
) -> Result<controller::Action, kube::Error> {
    info!("Got OfferDeployment event");
    Ok(controller::Action::await_change())
}

fn error_policy(
    _obj: Arc<OfferDeployment>,
    _err: &kube::Error,
    _ctx: Arc<()>,
) -> controller::Action {
    controller::Action::requeue(std::time::Duration::from_secs(60))
}
