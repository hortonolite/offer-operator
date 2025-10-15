mod crd;

use std::sync::Arc;
use futures::StreamExt;
use kube::{
    api::ListParams,
    runtime::controller::{self, Controller},
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

    Controller::new(api, ListParams::default())
        .run(reconcile, error_policy, Arc::new(()))
        .for_each(|res| async move {
            match res {
                Ok(obj) => info!("Reconciled: {:?}", obj),
                Err(err) => error!("Reconcile failed: {:?}", err),
            }
        })
        .await;

    Ok(())
}

async fn reconcile(_obj: Arc<OfferDeployment>, _ctx: Arc<()>) -> Result<(), kube::Error> {
    info!("Got OfferDeployment event");
    Ok(())
}

fn error_policy(_err: &kube::Error, _ctx: Arc<()>) -> controller::Action {
    controller::Action::requeue(std::time::Duration::from_secs(60))
}
