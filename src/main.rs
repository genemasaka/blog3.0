use yew::prelude::*;
use yew::{classes, html};
use image::io::Reader as ImageReader;
fn App() -> Html {
    let landing_img = ImageReader::open("./assets/blog3_asset.png").unwrap().decode().unwrap();
   
    html! {
        <>
        <div class={classes!("container")}>
        <div class={classes!("left")}>
        <h2 class={classes!("title")}>{"blog3.0"}</h2>
        <p class={classes!("welcome-text")}>{"Blog3.0 is the future of decentralized blogging. 
        It tokenizes users' blogs and allows for seamless 
        reading and commenting on the blockchain."}
        </p>
        <img class={classes!("lan-img")}src={landing_img}/>
        </div>
        <div class!{class="right"}
        <p>{"login with metamask to start your journey"}</p>
        <button>{"login"}</button>
        </div>
        </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
