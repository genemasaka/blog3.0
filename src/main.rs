use yew::prelude::*;
use yew::{classes, html};
use yew::platform::spawn_local;
use js_sys;
use web_sys::{Window, window};
use ethers::providers::{Provider, Http};
use ethers::{utils, prelude::*};
use ethers_core::rand::thread_rng;
use std::{path::Path, sync::Arc};
use serde::{Deserialize, Serialize};
use serde_json;
use tokio;

#[tokio::main(flavor = "current_thread")]
async fn connect_wallet() -> Result<(), Box<dyn std::error::Error>> {
    let window: Window = window().unwrap();
    let ethereum = js_sys::Reflect::get(&window, &"ethereum".into());
    

    if ethereum.is_ok() {
    let provider = Provider::<Http>::try_from("https://goerli.infura.io/v3/16087e2d4b5247d589382c9038b12f12")?;
    let accounts = provider.get_accounts().await?;
    let wallets: Vec<LocalWallet> = accounts.iter()
        .map(|_| LocalWallet::new(&mut thread_rng()).with_chain_id(5u64))
        .collect();
    } else {
    println!("Please install Metamask in your browser");
    }

    Ok(())
}
#[function_component]
fn App() -> Html {
   
    html! {
        <>
        <div class={classes!("container")}>
        <div  class={classes!("row")}>
        <div class={classes!("left")}>
        <h2 class={classes!("title")}>{"blog3.0"}</h2>
        <p class={classes!("welcome-text")}>{"Blog3.0 is the future of decentralized blogging. 
        It tokenizes users' blogs and allows for seamless 
        reading and commenting on the blockchain."}
        </p>
        <img class={classes!("lan-img")} src="./assets/blog3_asset.png" />
        </div>
        <div class={classes!("right")}>
        <h3 class={classes!("login-txt")}>{"login with metamask to start your journey"}</h3>
        <button>{"login"}</button>
        </div>
        </div>
        </div>
        </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
