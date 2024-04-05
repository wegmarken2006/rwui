# rwui
UI library based on [Bootstrap 5.3.3](https://getbootstrap.com/) and [Bootstrap Icons 1.11.3](https://icons.getbootstrap.com/).
Some charts are suppported based on [plotly 2.30.0](https://plotly.com/javascript/getting-started/)
UI is served on http://localhost, then the default browser can be automatically launched.
Websockets used for logic <-> UI messages. 
Intended for personal utility tools.

Still  under development.
###
### App folder structure:
```
|-myapp
|----src
|    | myapp.rs
|----static
|    | web2.css
|    |----bootstrap
|    |----bootstrap-icons
|    |----plotly
```
Copy the static folder from the [examples](./examples) here.
Optionally, use web2.css for further customizations.

### To compile the examples:
```
cd examples\ksink
cargo run
```

### Minimal example:
```rust
fn main() {
    let mut gc = rwui::GuiCfg::default();

    let (mut ctx, mut body) = gc.init("Mini");
    
	let bt1 = gc.button_new("primary", "Count");
	let lb1 = gc.label_new("0");

	let mut count = 0;
    let lb11 = lb1.clone();
	bt1.callback(&mut ctx, move |_, _| {
		count = count +1;
		let text = format!("{}", count);
		lb11.change_text(&text);
	});

	body.add(&lb1);
	body.add(&bt1);

    gc.run(ctx, &body); //close functions inside run

    gc.wait_key_from_console();
}
```
