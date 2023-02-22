use yew::prelude::*;
use yew::{classes, html};


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
