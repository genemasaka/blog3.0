use stylist::{yew::styled_component, style};
use yew::prelude::*;
struct Blog {
	token: String,
	title: String,
	writer: String,
	url: String,
}

#[styled_component(App)]
fn app()-> Html {
	let videos: Vec<String> = vec![];
	let card_style = style!(
		r#"
		box-shadow: 0 4px 8px 0 rgba(0,0,0,0.3);
		height: 200px;
		width: 30%;
		margin-top: 75px;
		border-radius: 18px;
		background-color: black;
		color: white;
		"#
		).expect("Failed to mount style");
	let main_content_style = style!(
			r#"
			height: 100%;
			width: 100%;
			display: flex;
			align-content: center;
			justify-content: center;
			text-align: center;
			"#
		).expect("Failed to mount style");
	let button_style = style!(
		r#"
		text-decoration: none;
		padding: 5px 10px 5px 10px;
		color: white;
		background-color: blue;
		border-radius: 13px;
		margin-top: 45px;
		"#
		).expect("Failed to mount style");
	html! {
		<>
		<div class = {main_content_style}>
		<div class={card_style}>
 	    <img />
  		<div class="card-body">
    	<h1 class="card-title">{"Welcome to blog3.0"}</h1>
    	<p class="card-text">{"Connect Wallet To Access the Site"}</p>
    	<a href="#" class={button_style}>{"Connect Wallet"}</a>
  		</div>
		</div>
		</div>
		</>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}