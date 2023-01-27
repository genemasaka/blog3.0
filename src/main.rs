use stylist::style;
use yew::prelude::*;
struct Blog {
	token: String,
	title: String,
	writer: String,
	url: String,
}

#[function_component(App)]
fn app()-> Html {
	let videos: Vec<String> = vec![];
	let card_style = style!(
		r#"
		height: 55vh;
		width: 40vw;
		background_color: black;
		color: white;
		border_radius: 24px;
		"#
		).expect("Failed to mount style");
	html! {
		<>
		<div class={card_style} style="width: 18rem;">
 	    <img />
  		<div class="card-body">
    	<h1 class="card-title">{"Welcome to blog3.0"}</h1>
    	<p class="card-text">{"Connect Wallet To Access the Site"}</p>
    	<a href="#" class="btn btn-primary">{"Connect Wallet"}</a>
  		</div>
		</div>
		</>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}