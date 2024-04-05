use super::{GuiCfg, Elem, ElType};


impl GuiCfg {
    fn ply_plotnumnum(
        &self,
        id: &str,
        x_vec: &Vec<f64>,
        y_vec: &Vec<f64>,
        p_type: &str,
        mode: &str,
        title: &str,
        x_title: &str,
        y_title: &str,
        width: i32,
        height: i32,
    ) -> Elem {
        let mut x_str = "[".to_string();
        let mut y_str = "[".to_string();

        for (ind, x_elem) in x_vec.iter().enumerate() {
            x_str = format!("{}{:7.2}, ", x_str, x_elem);
            y_str = format!("{}{:7.2}, ", y_str, y_vec[ind]);
        }
        x_str = format!("{}]", x_str);
        y_str = format!("{}]", y_str);

        let h_start = format!(
            r#"
            <div id="{}"></div>
    "#,
            id
        );

        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.html = h_start;
        e.id = id.to_string().clone();
        e.el_type = ElType::ParagraphT;
        e.tx_ws = Some(self.tx_ws.clone());

        e.js = format!(
            r#"
            PLOT{} = document.getElementById('{}');
            var data{} = [{{
                x: {},
                y: {},
                type: "{}",
                mode: "{}",
            }}];
            var layout{} = {{
                title: "{}",
                xaxis: {{title: "{}"}},
                yaxis: {{title: "{}"}},
                width: {},
                height: {}
              }};
        
            Plotly.newPlot( PLOT{}, data{}, layout{});            
        "#,
        id, id, id, x_str, y_str, p_type, mode, id, title, x_title, y_title, width, height, id, id, id
        );

        e
    }

    fn ply_plotstrnum(
        &self,
        id: &str,
        x_vec: &Vec<String>,
        y_vec: &Vec<f64>,
        p_type: &str,
        mode: &str,
        title: &str,
        x_title: &str,
        y_title: &str,
        width: i32,
        height: i32,
    ) -> Elem {
        let mut x_str = "[".to_string();
        let mut y_str = "[".to_string();

        for (ind, x_elem) in x_vec.iter().enumerate() {
            x_str = format!(r#"{}"{}", "#, x_str, x_elem);
            y_str = format!("{}{:7.2}, ", y_str, y_vec[ind]);

        }
        x_str = format!("{}]", x_str);
        y_str = format!("{}]", y_str);

        let h_start = format!(
            r#"
            <div id="{}"></div>
    "#,
            id
        );

        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.html = h_start;
        e.id = id.to_string().clone();
        e.el_type = ElType::ParagraphT;
        e.tx_ws = Some(self.tx_ws.clone());

        e.js = format!(
            r#"
            PLOT{} = document.getElementById('{}');
            var data{} = [{{
                x: {},
                y: {},
                type: "{}",
                mode: "{}",
            }}];
            var layout{} = {{
                title: "{}",
                xaxis: {{title: "{}"}},
                yaxis: {{title: "{}"}},
                width: {},
                height: {}
              }};
        
            Plotly.newPlot( PLOT{}, data{}, layout{});            
        "#,
        id, id, id, x_str, y_str, p_type, mode, id, title, x_title, y_title, width, height, id, id, id
        );

        e
    }

    fn ply_plotnumstr(
        &self,
        id: &str,
        x_vec: &Vec<f64>,
        y_vec: &Vec<String>,
        p_type: &str,
        mode: &str,
        title: &str,
        x_title: &str,
        y_title: &str,
        width: i32,
        height: i32,
    ) -> Elem {
        let mut x_str = "[".to_string();
        let mut y_str = "[".to_string();

        for (ind, x_elem) in x_vec.iter().enumerate() {
            x_str = format!("{}{:7.2}, ", x_str, x_elem);
            y_str = format!(r#"{}"{}", "#, y_str, y_vec[ind]);

        }
        x_str = format!("{}]", x_str);
        y_str = format!("{}]", y_str);

        let h_start = format!(
            r#"
            <div id="{}"></div>
    "#,
            id
        );

        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.html = h_start;
        e.id = id.to_string().clone();
        e.el_type = ElType::ParagraphT;
        e.tx_ws = Some(self.tx_ws.clone());

        e.js = format!(
            r#"
            PLOT{} = document.getElementById('{}');
            var data{} = [{{
                x: {},
                y: {},
                type: "{}",
                mode: "{}",
            }}];
            var layout{} = {{
                title: "{}",
                xaxis: {{title: "{}"}},
                yaxis: {{title: "{}"}},
                width: {},
                height: {}
              }};
        
            Plotly.newPlot( PLOT{}, data{}, layout{});            
        "#,
        id, id, id, x_str, y_str, p_type, mode, id, title, x_title, y_title, width, height, id, id, id
        );

        e
    }

    fn ply_plotnummulti(
        &self,
        id: &str,
        y_vec: &Vec<Vec<f64>>,
        trace_names: &Vec<&str>,
        p_type: &str,
        mode: &str,
        title: &str,
        y_title: &str,
        width: i32,
        height: i32,
    ) -> Elem {

        let mut data = "[".to_string();

        for line in 0..y_vec.len() {
            let mut y_str = "[".to_string();
            for ind in 0..y_vec.len() {
                y_str = format!("{}{:7.2}, ", y_str, y_vec[line][ind]);                
            }
            y_str = format!("{}]", y_str);
            let datum = format!(
                r#"
                {{
                    y: {}, 
                    type: "{}",
                    mode: "{}",
                    name: "{}",
                }},
        "#,
            y_str, p_type, mode, trace_names[line]
            );
            data = format!("{}{}", data, datum);
        }
        data = format!("{}]", data);

        let h_start = format!(
            r#"
            <div id="{}"></div>
    "#,
            id
        );

        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.html = h_start;
        e.id = id.to_string().clone();
        e.el_type = ElType::ParagraphT;
        e.tx_ws = Some(self.tx_ws.clone());

        e.js = format!(
            r#"
            PLOT{} = document.getElementById('{}');
            var data{} = 
                {}
            ;
        
            var layout{} = {{
                title: "{}",
                yaxis: {{title: "{}"}},
                width: {},
                height: {}
              }};
        
            Plotly.newPlot( PLOT{}, data{}, layout{});
            "#,
            id, id, id, data, id, title, y_title, width, height, id, id, id
        );

        e
    }

    fn ply_plotnumnummulti(
        &self,
        id: &str,
        x_vec: &Vec<f64>,
        y_vec: &Vec<Vec<f64>>,
        trace_names: &Vec<&str>,
        p_type: &str,
        mode: &str,
        title: &str,
        x_title: &str,
        y_title: &str,
        width: i32,
        height: i32,
    ) -> Elem {

        let mut data = "[".to_string();

        for line in 0..y_vec.len() {
            let mut x_str = "[".to_string();
            let mut y_str = "[".to_string();
            for ind in 0..y_vec.len() {
                x_str = format!("{}{:7.2}, ", x_str, x_vec[ind]);     
                y_str = format!("{}{:7.2}, ", y_str, y_vec[line][ind]);     

            }
            x_str = format!("{}]", x_str);
            y_str = format!("{}]", y_str);
            let datum = format!(
                r#"
                {{
                    x: {}, 
                    y: {}, 
                    type: "{}",
                    mode: "{}",
                    name: "{}",
                }},
        "#,
            x_str, y_str, p_type, mode, trace_names[line]
            );
            data = format!("{}{}", data, datum);
        }
        data = format!("{}]", data);

        let h_start = format!(
            r#"
            <div id="{}"></div>
    "#,
            id
        );

        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.html = h_start;
        e.id = id.to_string().clone();
        e.el_type = ElType::ParagraphT;
        e.tx_ws = Some(self.tx_ws.clone());

        e.js = format!(
            r#"
            PLOT{} = document.getElementById('{}');
            var data{} = 
                {}
            ;
        
            var layout{} = {{
                title: "{}",
                xaxis: {{title: "{}"}},
                yaxis: {{title: "{}"}},
                width: {},
                height: {}
              }};
        
            Plotly.newPlot( PLOT{}, data{}, layout{});
            "#,
            id, id, id, data, id, title, x_title, y_title, width, height, id, id, id
        );

        e
    }


    pub fn plot_line_new(
        &self,
        x: &Vec<f64>,
        y: &Vec<f64>,
        title: &str,
        x_title: &str,
        y_title: &str,
        width: i32,
        height: i32,
    ) -> Elem {
        let id = self.id_new();

        self.ply_plotnumnum(&id, &x, &y, "", "lines", title, x_title, y_title, width, height) 
    }

    pub fn plot_scatter_new(
        &self,
        x: &Vec<f64>,
        y: &Vec<f64>,
        title: &str,
        x_title: &str,
        y_title: &str,
        width: i32,
        height: i32,
    ) -> Elem {
        let id = self.id_new();

        self.ply_plotnumnum(&id, &x, &y, "scatter", "markers", title, x_title, y_title, width, height) 
    }

    pub fn plot_vbar_new(
        &self,
        x: &Vec<String>,
        y: &Vec<f64>,
        title: &str,
        x_title: &str,
        y_title: &str,
        width: i32,
        height: i32,
    ) -> Elem {
        let id = self.id_new();

        self.ply_plotstrnum(&id, &x, &y, "bar", "s", title, x_title, y_title, width, height) 
    }

    pub fn plot_hbar_new(
        &self,
        x: &Vec<f64>,
        y: &Vec<String>,
        title: &str,
        x_title: &str,
        y_title: &str,
        width: i32,
        height: i32,
    ) -> Elem {
        let id = self.id_new();

        self.ply_plotnumstr(&id, &x, &y, "bar", "s", title, x_title, y_title, width, height) 
    }


    pub fn plot_line_multiy_new(
        &self,
        x: &Vec<f64>,
        y: &Vec<Vec<f64>>,
        title: &str,
        trace_names: &Vec<&str>,
        x_title: &str,
        y_title: &str,
        width: i32,
        height: i32,
    ) -> Elem {
        let id = self.id_new();

        self.ply_plotnumnummulti(&id, x, y, trace_names, "", "lines", title, x_title, y_title, width, height) 
    }
    
    pub fn plot_boxplot_new(
        &self,
        y: &Vec<Vec<f64>>,
        title: &str,
        trace_names: &Vec<&str>,
        y_title: &str,
        width: i32,
        height: i32,
    ) -> Elem {
        let id = self.id_new();

        self.ply_plotnummulti(&id, y, trace_names, "box", "", title, y_title, width, height) 
    }
    }

    impl Elem {
        /// plot_redrawy updates a chart with new numeric y array
        pub fn plot_redrawy(&self, y: &Vec<f64>) {
            let mut to_send = format!("PREDRAWY@{}@{}", self.id, y.len());
            for y_elem in y {
                to_send = format!("{}@{:7.2}", to_send, y_elem);     
            }
            let _ = self.tx_ws.as_ref().unwrap().send(to_send);
    
        }

        /// plot_redrawxsy updates a chart with new string x array
        /// and new numeric y array
        pub fn plot_redrawxsy(&self, x: &Vec<String>, y: &Vec<f64>) {
            let mut to_send = format!("PREDRAWXSY@{}@{}", self.id, y.len());
            for y_elem in y {
                to_send = format!("{}@{:7.2}", to_send, y_elem);     
            }
            for x_elem in x {
                to_send = format!("{}@{}", to_send, x_elem);     
            }

            let _ = self.tx_ws.as_ref().unwrap().send(to_send);    
        }
    
    }
/* 
    // PlotRedrawY updates a chart with new numeric y array
func (gc *GuiCfg) PlotRedrawY(el Elem, y []float64) {
	//Plotly.newPlot(element,charData,layout);
	if gc.Body.gs != nil {
		toSend := Sprintf("PREDRAWY@%s@%d", el.id, len(y))
		for _, yElem := range y {
			toSend = Sprintf("%s@%7.2f", toSend, yElem)
		}
		gc.mutex.Lock()
		defer gc.mutex.Unlock()
		gc.Body.gs.WriteMessage(websocket.TextMessage, []byte(toSend))
	} else {
		Println("Failed Redraw, Set", gc.Body.id, "Callback!")
	}
}

// PlotRedrawXsY updates a chart with new string x array
// and new numeric y array
func (gc *GuiCfg) PlotRedrawXsY(el Elem, x []string, y []float64) {
	//Plotly.newPlot(element,charData,layout);
	if gc.Body.gs != nil {
		toSend := Sprintf("PREDRAWXSY@%s@%d", el.id, len(y))
		for _, yElem := range y {
			toSend = Sprintf("%s@%7.2f", toSend, yElem)
		}
		for _, xElem := range x {
			toSend = Sprintf("%s@%s", toSend, xElem)
		}

		gc.mutex.Lock()
		defer gc.mutex.Unlock()
		gc.Body.gs.WriteMessage(websocket.TextMessage, []byte(toSend))
	} else {
		Println("Failed Redraw, Set", gc.Body.id, "Callback!")
	}
}
 */




