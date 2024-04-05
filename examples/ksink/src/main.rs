use rwui::exec_if_modal_selected;

fn main() {
    let mut gc = rwui::GuiCfg::default();
    gc.plot_included = true;

    let (mut ctx, mut body) = gc.init("TEST RWUI");
    body.set_background_image("abstract.jpg", 95);
    let mut tabs = gc.tabs_new(vec!["tab1", "tab2", "tab3"]);

    let bt1 = gc.button_withicon_new("primary", "brush", "Start");
    let lb1 = gc.label_new("Start TextArea write loop");

    let bt2 = gc.button_withicon_new("secondary", "brush-fill", "Change");
    let lb2 = gc.label_new("Change text background color");

    let bt3 = gc.button_withicon_new("success", "box-arrow-up-right", "Change");
    let lb3 = gc.label_new("Change text size");

    let lb4 = gc.label_new("Input text with Modal");
    let it4 = gc.inputtext_new("text");

    let lb5 = gc.label_new("Select font family");

    let dd5 = gc.dropdown_new(
        "secondary",
        "Font Family",
        vec!["arial", "verdana", "monospace"],
    );

    let md1 = gc.modal_new("TEXT INPUT", "Are you sure", "yes", "no");

    let mut ta1 = gc.textarea_new(12);
    ta1.set_background_color("#ffe6e6");
    ta1.set_color("blue");
    ta1.set_font_size("small");
    ta1.set_font_family("monospace");

    let mut cd1 = gc.card_new("Kitchen Sink", "Elements");
    cd1.set_background_color("#eeffee");

    let md11 = md1.clone();
    md1.sub_elems[0].callback(&mut ctx, move |_str1, _int1| {
        md11.modal_select(true);
    });

    let md12 = md1.clone();
    md1.sub_elems[1].callback(&mut ctx, move |_str1, _int1| {
        md12.modal_select(false);
    });

    let ta11 = ta1.clone();
    let md13 = md1.clone();
    it4.callback(&mut ctx, move |str1, _int1| {
        md13.modal_show();

        let md14 = md13.clone();
        let text = format!("From Input field: {}\n", str1);
        let ta12 = ta11.clone();

        //write text area only if modal yes pressed
        exec_if_modal_selected!(md14, ta12.write_textarea(&text));
    });

    let ta12 = ta1.clone();
    let bt11 = bt1.clone();
    bt1.callback(&mut ctx, move |_, _| {
        let ta13 = ta12.clone();
        //write only if modal yes pressed
        std::thread::spawn(move || {
            let mut ind = 0;
            loop {
                ind = ind + 1;
                let text = format!("{}: All work and no play ...\n", ind);
                ta13.write_textarea(&text);
                std::thread::sleep(std::time::Duration::from_millis(3000));
            }
        });
        bt11.change_to_disable();
    });

    let ta13 = ta1.clone();
    dd5.callback(&mut ctx, move |str1, _int1| {
        ta13.change_font_family(str1);
    });

    let bt21 = bt2.clone();
    let ta14 = ta1.clone();
    bt2.callback(&mut ctx, move |_, _| {
        ta14.change_background_color("#66ffff");
        bt21.change_text("Changed");
    });

    let bt31 = bt3.clone();
    let ta15 = ta1.clone();
    bt3.callback(&mut ctx, move |_, _| {
        ta15.change_font_size("large");
        bt31.change_text("Changed");
    });

    let _fi1 = gc.file_input_new("Open");

    let mut ct1 = gc.container_new();
    let mut r1 = gc.row_new();
    let mut r11 = gc.row_new();
    let mut r12 = gc.row_new();
    let mut r13 = gc.row_new();
    let mut r14 = gc.row_new();
    let mut r15 = gc.row_new();
    let mut c1 = gc.col_new();
    let mut c2 = gc.col_new();
    let mut c11 = gc.col_new();
    let mut c12 = gc.col_new();
    let mut c21 = gc.col_new();
    let mut c22 = gc.col_new();
    let mut c31 = gc.col_new();
    let mut c32 = gc.col_new();
    let mut c41 = gc.col_new();
    let mut c42 = gc.col_new();
    let mut c51 = gc.col_new();
    let mut c52 = gc.col_new();
    c11.add(&lb1);
    c12.add(&bt1);
    c21.add(&lb2);
    c22.add(&bt2);
    c31.add(&lb3);
    c32.add(&bt3);
    c41.add(&lb4);
    c42.add(&it4);
    c51.add(&lb5);
    c52.add(&dd5);
    r11.add(&c11);
    r11.add(&c12);
    r12.add(&c21);
    r12.add(&c22);
    r13.add(&c31);
    r13.add(&c32);
    r14.add(&c41);
    r14.add(&c42);
    r15.add(&c51);
    r15.add(&c52);
    c1.add(&r11);
    c1.add(&r12);
    c1.add(&r13);
    c1.add(&r14);
    c1.add(&r15);

    c2.add(&ta1);

    r1.add(&c1);
    r1.add(&c2);

    ct1.add(&r1); //container
    cd1.add(&ct1);

    //Fisrt tab content
    tabs.sub_elems[0].add(&cd1);

    let img1 = gc.image_new("abstract.jpg", 50, 50);
    let img2 = gc.image_new("abstract.jpg", 100, 100);
    let img3 = gc.image_new("abstract.jpg", 200, 200);
    let img4 = gc.image_new("abstract.jpg", 300, 100);

    let rs1 = gc.rangeslider_new(50.0, 0.0, 100.0, 1.0);
    let rd_choices = vec!["do nothing", "abstract.jpg", "bricks.jpg"];
    let rd1 = gc.radio_new(&rd_choices, 1);
    let pb1 = gc.pillbadge_new("danger", "50");

    let pb11 = pb1.clone();
    rs1.callback(&mut ctx, move |str1, _| {
        pb11.change_text(str1);
    });

    let img41 = img4.clone();
    rd1.callback(&mut ctx, move |_, int1| {
        if int1 > 0 {
            img41.change_image(rd_choices[int1 as usize]);
        }
    });

    let fi1 = gc.file_input_new("Open");
    let lb11 = gc.label_new("No file");

    let lb111 = lb11.clone();
    fi1.callback(&mut ctx, move |str1, _| {
        lb111.change_text(str1);
    });

    let mut par1 = gc.paragraph_new();
    par1.add(&img1);
    par1.add(&img2);
    par1.add(&img3);
    par1.add(&img4);

    let colsp21 = rwui::ColSpans {
        elems: vec![rd1, par1],
        spans: vec![4, 0],
    };
    let colsp22 = rwui::ColSpans {
        elems: vec![pb1, rs1],
        spans: vec![2, 0],
    };
    let colsp23 = rwui::ColSpans {
        elems: vec![fi1, lb11],
        spans: vec![0, 2],
    };
    let ct2 = gc.grid_new(vec![colsp21, colsp22, colsp23]);
    tabs.sub_elems[1].add(&ct2);
    tabs.sub_elems[1].set_background_color("white");

    let x2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mut y2 = vec![1.0, 2.0, 4.0, 8.0, 16.0];
    let y2_original = vec![1.0, 2.0, 4.0, 8.0, 16.0];
    let mut xs2 = vec!["aa".to_string(), "bb".to_string(), "cc".to_string(), "dd".to_string(), "ee".to_string()];
    let ym2 = vec![
        vec![1.0, 2.0, 4.0, 8.0, 16.0],
        vec![2.0, 4.0, 8.0, 16.0, 18.0],
        vec![5.0, 8.0, 16.0, 18.0, 19.0],
    ];

    let pl1 = gc.plot_line_new(&x2, &y2, "Line", "xaxis", "yaxis", 250, 250);
    let pl2 = gc.plot_scatter_new(&x2, &y2, "Scatter", "xaxis", "yaxis", 250, 250);
    let pl3 = gc.plot_vbar_new(&xs2, &y2, "VBar", "xaxis", "yaxis", 250, 250);
    let pl4 = gc.plot_hbar_new(&y2_original, &xs2, "HBar", "xaxis", "yaxis", 250, 250);
    let pl5 = gc.plot_line_multiy_new(
        &x2,
        &ym2,
        "Multi",
        &vec!["traceA", "tB", "tC"],
        "xaxis",
        "yaxis",
        250,
        250,
    );

    let pl6 = gc.plot_boxplot_new(
        &ym2,
        "Boxplot",
        &vec!["setA", "sB", "sC"],
        "yaxis",
        250,
        250,
    );
    let rs2 = gc.rangeslider_new(1.0, 1.0, 100.0, 1.0);

    let mut upper = true;
    let pl11 = pl1.clone();
    let pl21 = pl2.clone();
    let pl31 = pl3.clone();

	rs2.callback(&mut ctx, move |_, int1| {
        for (ind, y_elem) in y2_original.iter().enumerate() {
			y2[ind] = y_elem + int1 as f64;
            pl11.plot_redrawy(&y2);
            pl21.plot_redrawy(&y2);

			if upper {
                for ind in 0..xs2.len()  {
                    let x_elem = xs2[ind].to_uppercase();
					xs2[ind] = x_elem;
				}
				upper = false;
			} else {
				for ind in 0..xs2.len()  {
					let x_elem = xs2[ind].to_lowercase();
                    xs2[ind] = x_elem;
				}
				upper = true;
			}
            pl31.plot_redrawxsy(&xs2, &y2);

		}
	});


	let mut ct3 = gc.container_new();
	let mut r1t3 = gc.row_new();
	let mut c1t3 = gc.col_new();
	let mut c2t3 = gc.col_new();
	let mut c3t3 = gc.col_new();
	c1t3.add(&pl1);
	c1t3.add(&pl2);
	c1t3.add(&rs2);
	c2t3.add(&pl3);
	c2t3.add(&pl4);
	c3t3.add(&pl5);
	c3t3.add(&pl6);
	r1t3.add(&c1t3);
	r1t3.add(&c2t3);
	r1t3.add(&c3t3);
	ct3.add(&r1t3);

	tabs.sub_elems[2].add(&ct3);
    tabs.sub_elems[2].set_background_color("white");

    body.add(&tabs);
    body.add(&md1);

    gc.run(ctx, &body); //close functions inside run

    gc.wait_key_from_console();
}
