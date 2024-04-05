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
