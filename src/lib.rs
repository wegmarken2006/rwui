#[macro_use]
mod rwplot;

use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};

use std::net::TcpListener;

use tungstenite::{accept, Message};

use std::sync::Mutex;
use std::thread::spawn;

use crossbeam_channel::{select, Receiver, Sender};

#[derive(Debug, Clone, PartialEq)]
enum ElType {
    ButtonT,
    TextAreaT,
    LabelT,
    RowT,
    ColT,
    BodyT,
    ITextT,
    TabsT,
    TPaneT,
    ParagraphT,
    DDownT,
    CardT,
    ModalT,
    ImageT,
    RSliderT,
    PBadgeT,
    RadioT,
    ContainerT,
    FileInputT,
}

impl Default for ElType {
    fn default() -> Self {
        ElType::BodyT
    }
}

#[derive(Debug, Clone)]
pub struct Elem {
    el_type: ElType,
    h_start: String,
    h_end: String,
    html: String,
    js: String,
    id: String,
    pub sub_elems: Vec<Elem>,
    sub_start: String,
    sub_end: String,
    tx_ws: Option<Sender<String>>, //channel for sending to client via websock
    tx_ws_for_ta: Option<Sender<String>>, //channel dedicated to websock messages for text area
    pub tx_modal: Option<Sender<bool>>,
    pub rx_modal: Option<Receiver<bool>>,
}

pub struct Callback {
    el: Elem,
    el_type: ElType,
    id: String,
    fun: Box<dyn FnMut(&str, i32) + std::marker::Send + 'static>,
}

#[derive(Default)]
pub struct Callbacks {
    cbs: Vec<Callback>,
}

/// Colspans is a struct used to build grids
pub struct ColSpans {
	pub elems: Vec<Elem>,
	pub spans: Vec<i32>
}

#[derive(Debug)]
pub struct GuiCfg {
    fh: File,
    fh_name: String,
    fjs: File,
    //fcss: File,
    id_cnt: Mutex<i32>,
    serve_url: String,
    ws_url: String,
    tx_ws: Sender<String>,
    rx_ws: Receiver<String>,
    ws_listener: Option<TcpListener>,
    pub browser_start: bool,
    pub plot_included: bool,
    pub exit_on_window_close: bool,
}

impl Default for Elem {
    fn default() -> Elem {
        let e = Elem {
            el_type: ElType::default(),
            h_start: String::default(),
            h_end: String::default(),
            html: String::default(),
            js: String::default(),
            id: String::default(),
            sub_elems: vec![],
            sub_start: String::default(),
            sub_end: String::default(),
            tx_ws: None,
            tx_ws_for_ta: None,
            tx_modal: None,
            rx_modal: None,
        };
        e
    }
}

impl Elem {
    pub fn add(&mut self, n: &Elem) {
        self.html = format!("{}{}{}", self.html, n.html, n.h_end);
        self.js = format!("{}{}", self.js, n.js);

        self.html = format!("{}{}", self.html, n.sub_start);

        for se in &n.sub_elems {
            self.html = format!("{}{}{}", self.html, se.html, se.h_end);
            self.js = format!("{}{}", self.js, se.js);
        }
        self.html = format!("{}{}", self.html, n.sub_end);
    }

    pub fn callback(
        &self,
        cbs: &mut Callbacks,
        fun: impl FnMut(&str, i32) + std::marker::Send + 'static,
    ) {
        let addr = format!("/{}", self.id);
        //println!("callback: *{}*", addr);
        let el_type = self.el_type.clone();

        let cb = Callback {
            el: self.clone(),
            el_type: el_type,
            id: addr,
            fun: Box::new(fun),
        };
        cbs.cbs.push(cb);
    }

    /// set_background_image: the image must reside in the static/ folder.
    /// Opacity is expressed as %; pass 100 for no transparency.
    pub fn set_background_image(&mut self, filename: &str, opacity_perc: i32) {
        //let mut js = String::default();
        let js: String;
        if self.el_type == ElType::BodyT {
            js = format!(
                r#"
        document.body.style.backgroundImage = "url('static/{}')";	
		document.body.style.backgroundSize = "cover";	
		document.body.style.opacity = "{}{}";	
        "#,
                filename, opacity_perc, '%'
            );
        } else {
            js = format!(
                r#"
		var item = document.getElementById("{}");
		item.style.backgroundImage = "url('static/{}')";		
		item.style.backgroundSize = "cover";	
		item.style.opacity = "{}{}";		q
        "#,
                self.id, filename, opacity_perc, '%'
            );
        }

        self.js = format!("{}{}", self.js, js);
    }

    pub fn modal_show(&self) {
        if self.el_type == ElType::ModalT {
            let tx_ws = self.tx_ws.clone();
            let to_send = format!("MODALSHOW@{}@{}", self.id, "dummy");
            let _ = tx_ws.as_ref().unwrap().send(to_send);
        }
    }

    pub fn write_textarea(&self, text: &str) {
        if self.el_type == ElType::TextAreaT {
            let _ = self.tx_ws_for_ta.as_ref().unwrap().send(text.to_string());
        }
    }

    pub fn modal_select(&self, flag: bool) {
        if self.el_type == ElType::ModalT {
            let _ = self.tx_modal.as_ref().unwrap().send(flag);
        }
    }

    pub fn modal_selected(&self) -> bool {
        let mut flag = false;
        if self.el_type == ElType::ModalT {
            flag = self.rx_modal.as_ref().unwrap().recv().unwrap();
        }
        flag
    }

    /// set_background_color sets an element background color.
    pub fn set_background_color(&mut self, text: &str) {
        let js: String;
        if self.el_type == ElType::BodyT {
            js = format!(
                r#"
            document.body.style.backgroundColor = "{}";		
            "#,
                text
            );
        } else {
            js = format!(
                r#"
            var item = document.getElementById("{}");
            item.style.backgroundColor = "{}";		
            "#,
                self.id, text
            );
        }
        self.js = format!("{}{}", self.js, js);
    }

    /// set_color sets an element foreground color.
    pub fn set_color(&mut self, text: &str) {
        let js: String;
        js = format!(
            r#"
        var item = document.getElementById("{}");
        item.style.color = "{}";		
        "#,
            self.id, text
        );
        self.js = format!("{}{}", self.js, js);
    }

    /// set_font_size sets an element font size.
    pub fn set_font_size(&mut self, text: &str) {
        let js: String;
        js = format!(
            r#"
        var item = document.getElementById("{}");
        item.style.fontSize = "{}";		
            "#,
            self.id, text
        );
        self.js = format!("{}{}", self.js, js);
    }

    pub fn set_font_family(&mut self, text: &str) {
        let js: String;
        js = format!(
            r#"
        var item = document.getElementById("{}");
        item.style.fontFamily = "{}";		
            "#,
            self.id, text
        );
        self.js = format!("{}{}", self.js, js);
    }

    /// change_to_disable changes on the run an element status to disable.
    pub fn change_to_disable(&self) {
        let to_send = format!("ENABLE@{}@{}", self.id, "DISABLE");
        let _ = self.tx_ws.as_ref().unwrap().send(to_send);
    }

    /// change_to_disable changes on the run an element status to disable.
    pub fn change_to_enable(&self) {
        let to_send = format!("ENABLE@{}@{}", self.id, "ENABLE");
        let _ = self.tx_ws.as_ref().unwrap().send(to_send);
    }

    /// click forces a click.
    pub fn click(&self) {
        let to_send = format!("CLICK@{}", self.id);
        let _ = self.tx_ws.as_ref().unwrap().send(to_send);
    }

    /// change_image changes the image element src with the file_name passed.
    pub fn change_image(&self, file_name: &str) {
        let to_send = format!("IMAGE@{}@{}", self.id, file_name);
        let _ = self.tx_ws.as_ref().unwrap().send(to_send);
    }

    /// change_text changes on the run an element text.
    pub fn change_text(&self, text: &str) {
        let to_send: String;
        if self.el_type == ElType::ButtonT {
            to_send = format!("TEXT@{}text@{}", self.id, text);
        } else {
            to_send = format!("TEXT@{}@{}", self.id, text);
        }

        let _ = self.tx_ws.as_ref().unwrap().send(to_send);
    }

    /// change_font_family changes on the run an element font family.
    pub fn change_font_family(&self, text: &str) {
        let to_send = format!("FONTFAMILY@{}@{}", self.id, text);
        let _ = self.tx_ws.as_ref().unwrap().send(to_send);
    }

    /// change_font_size changes on the run an element font soze.
    pub fn change_font_size(&self, text: &str) {
        let to_send = format!("FONTSIZE@{}@{}", self.id, text);
        let _ = self.tx_ws.as_ref().unwrap().send(to_send);
    }

    /// change_color changes on the run an element color.
    pub fn change_color(&self, text: &str) {
        let to_send = format!("COLOR@{}@{}", self.id, text);
        let _ = self.tx_ws.as_ref().unwrap().send(to_send);
    }

    /// change_background_color changes on the run an element background color.
    pub fn change_background_color(&self, text: &str) {
        let to_send = format!("BCOLOR@{}@{}", self.id, text);
        let _ = self.tx_ws.as_ref().unwrap().send(to_send);
    }
}

impl Default for GuiCfg {
    fn default() -> GuiCfg {
        if std::fs::metadata("static").is_err() {
            panic!(
                "\n* Folder ./static missing\n* Make ./static folder and copy bootstrap inside."
            );
        }
        let fh_name = "static/index.html";
        let fh = File::create(fh_name).unwrap();
        let fjs = File::create("static/web2.js").unwrap();
        //let fcss = File::create("static/web2.css").unwrap();

        let (tx_ws, rx_ws): (Sender<String>, Receiver<String>) = crossbeam_channel::unbounded();

        let gc = GuiCfg {
            fh: fh,
            fh_name: fh_name.to_string(),
            fjs: fjs,
            //fcss: fcss,
            id_cnt: Mutex::new(0),
            browser_start: false,
            exit_on_window_close: false,
            plot_included: false,
            serve_url: String::default(),
            ws_url: "127.0.0.1:9000".to_string(),
            ws_listener: None,
            tx_ws: tx_ws,
            rx_ws: rx_ws,
        };

        gc
    }
}

impl GuiCfg {
    pub fn run(&mut self, mut cbs: Callbacks, body: &Elem) {
        self.fh.write(body.html.as_bytes()).unwrap();
        self.fh.write(body.h_end.as_bytes()).unwrap();

        //TODO: for now embed js in html
        self.fh.write("\n<script>\n".as_bytes()).unwrap();
        self.fh.write(body.js.as_bytes()).unwrap();
        self.fh
            .write("\n</script>\n</body>\n</html>\n".as_bytes())
            .unwrap();

        //

        self.fjs.write(body.js.as_bytes()).unwrap();

        let listener = TcpListener::bind("127.0.0.1:0").expect("tcp server error");
        let addr = listener.local_addr().unwrap().clone();

        //let ws_listener = TcpListener::bind(&self.ws_url).expect("tcp server error");
        //let ws_addr = ws_listener.local_addr().unwrap().clone();

        //ws listener bound during init(), url is already known and saved in self.ws_url
        let ws_listener = self.ws_listener.as_ref().unwrap().try_clone().unwrap();

        let fh_name = self.fh_name.clone();
        let content = std::fs::read_to_string(fh_name).unwrap();
        //let mut contents = format!("{}{}", body.html, body.h_end);

        let rx_ws = self.rx_ws.clone();
        spawn(move || {
            for stream in ws_listener.incoming() {
                let stream = stream.unwrap();

                let mut websocket = accept(stream).unwrap();

                loop {
                    select! {
                        recv(rx_ws) -> rx_msg =>  {
                            websocket.send(Message::Text(rx_msg.unwrap())).unwrap();
                        }
                        default => {
                        let msg = websocket.read().unwrap_or_else(|_| {std::process::exit(0);});
                        let msg_str = msg.to_string();
                        let msgs: Vec<&str> = msg_str.split("@@").collect();
                        if msgs[0].starts_with("/") {
                            for cb in &mut cbs.cbs {
                                if cb.id == msgs[0] {
                                    let fun3 = cb.fun.as_mut();
                                    let el_type = cb.el_type.clone();

                                    if el_type == ElType::ButtonT {
                                        fun3("", 0);
                                    } else if  el_type == ElType::ITextT || el_type == ElType::DDownT || el_type == ElType::RSliderT || el_type == ElType::RadioT || el_type == ElType::FileInputT {
                                        if el_type == ElType::DDownT {
                                            cb.el.change_text(msgs[1]);
                                        }
                                        if el_type == ElType::ITextT || el_type == ElType::DDownT || el_type == ElType::FileInputT {
                                            let mut str1 = msgs[1];
                                            if el_type == ElType::FileInputT {
                                                str1 = std::path::Path::new(msgs[1]).file_name().unwrap().to_str().unwrap();                                                
                                            }
                                            fun3(str1, 0)
                                        } else if el_type == ElType::RSliderT || el_type == ElType::RadioT {
                                            let int_value =  msgs[1].parse::<i32>().unwrap();
                                            fun3(msgs[1], int_value);
                                        }

                                        //println!("{}", msgs[1]);
                                    }

                                }
                            }
                        }
                    }

                    }
                }
            }
        });

        spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();

                let buf_reader = BufReader::new(&mut stream);

                let http_request: Vec<_> = buf_reader
                    .lines()
                    .map(|result| result.unwrap())
                    .take_while(|line| !line.is_empty())
                    .collect();

                //println!("Request: {:#?}", http_request);

                let ok_response = "HTTP/1.1 200 OK\r\n\r\n";

                let strs: Vec<&str> = http_request[0].split_whitespace().collect();
                if strs[0] == "GET" {
                    if strs[1].starts_with("/static/") {
                        let path = strs[1][1..].to_string(); //cut initial /

                        let mut static_content = Vec::new();
                        let mut file = File::open(&path).expect("Unable to open file");
                        file.read_to_end(&mut static_content)
                            .expect("Unable to read");
                        stream.write_all(ok_response.as_bytes()).unwrap();
                        stream.write_all(&static_content).unwrap();

                        continue;
                    }
                }

                let response = format!("{}{}", ok_response, content);

                stream.write_all(response.as_bytes()).unwrap();
            }
            //stream.flush().unwrap();
        });

        let addr_str = format!("http://{:?}", addr);
        self.serve_url = addr_str;
        //self.ws_url = format!("{}", ws_addr);

        println!("Serving on {}, ws on {}", &self.serve_url, &self.ws_url);

        //std::thread::sleep(std::time::Duration::from_millis(100));

        webbrowser::open(&self.serve_url).unwrap();
    }

    pub fn wait_key_from_console(&mut self) {
        let stdin = io::stdin();
        let mut reader = stdin.lock();

        println!("Press:\n q<Enter> to exit");

        loop {
            let mut text = String::new();
            reader.read_line(&mut text).unwrap();

            match text.trim() {
                "q" | "Q" => std::process::exit(0),
                _ => continue,
            }
        }
    }

    //<link href="/static/bootstrap/css/bootstrap.css" rel="stylesheet" media="screen">
    //<link href="/static/web2.css" rel="stylesheet">
    //<script type="text/javascript" src="/static/bootstrap/js/bootstrap.bundle.js"></script>
    pub fn init(&mut self, title: &str) -> (Callbacks, Elem) {
        let ws_listener = TcpListener::bind("127.0.0.1:0").expect("tcp server error");
        self.ws_url = format!("{}", ws_listener.local_addr().unwrap().clone());
        self.ws_listener = Some(ws_listener);

        let h_start = format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
        
            <head>
            <title>{}</title>
            <meta name="viewport" content="width=device-width, initial-scale=1">
        
            <!-- -->
            <link href="static/bootstrap/css/bootstrap.css" rel="stylesheet" media="screen">
            <link href="web2.css" rel="stylesheet">            
            <!-- 
            <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN" crossorigin="anonymous"> 
            -->
        
            </head>
        
            <body>
            <!-- -->
            <script type="text/javascript" src="static/bootstrap/js/bootstrap.bundle.js"></script>            
            <!--
            <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js" integrity="sha384-YvpcrYf0tY3lHB60NNkmXc5s9fDVZLESaAA55NDzOxhy9GkcIdslK1eN7N6jIeHz" crossorigin="anonymous"></script> 
            -->
            "#,
            title
        );

        let mut plot_script = "".to_string();
        if self.plot_included {
            plot_script = format!(
                r#"<script type="text/javascript" src="/static/plotly/plotly-2.30.0.min.js"></script> "#
            );
        }
        let h_end = format!(
            r#"
            {}"#,
            plot_script
        );
        //TODO
        /*
        let h_end = format!(
            r#"
            </body>

            {}
            <script type="text/javascript" src="/static/web2.js"></script>
            </html>"#,
            plot_script
        );
        */

        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.h_end = h_end;
        e.html = h_start;
        e.id = "body".to_string();
        e.el_type = ElType::BodyT;
        e.tx_ws = Some(self.tx_ws.clone());

        e.js = format!(
            r#"
            var addr1 = "ws://" + "{}";
            conn_ws = new WebSocket(addr1);
            conn_ws.onmessage = function (evt) {{
                var messages = evt.data.split('@');
                var type = messages[0];
                var id = messages[1];
                var item = document.getElementById(id);
                if (type === "TEXT") {{
                    item.innerHTML = messages[2];
                }}
                else if (type === "COLOR") {{
                    var color = messages[2];
                    item.style.color = color;
                }}
                else if (type === "BCOLOR") {{
                    var color = messages[2];
                    item.style.backgroundColor  = color;
                }}
                else if (type === "FONTSIZE") {{
                    var fsize = messages[2];
                    item.style.fontSize  = fsize;
                }}
                else if (type === "FONTFAMILY") {{
                    var font = messages[2];
                    item.style.fontFamily  = font;
                }}
                else if (type === "MODALSHOW") {{
                    var modal = new bootstrap.Modal(item, {{
                        keyboard: false
                      }}); 
                    modal.show();
                }}
                else if (type === "ENABLE") {{
                    var enable = messages[2];
                    if (enable === "ENABLE") {{
                        item.disabled = false;
                    }}
                    else  {{
                        item.disabled = true;
                    }}
                }}
                else if (type === "IMAGE") {{
                    src = 'static/' + messages[2]; 
                    item.src = src;
                }}
                else if (type === "PREDRAWY") {{
                    var len = parseInt(messages[2], 10);
                    var yVec = new Float32Array(len);
                    for (ind = 0; ind < len; ind++) {{
                        yVec[ind] = parseFloat(messages[3+ind]);
                    }}
                    window["data" + id][0].y = yVec;
                    Plotly.newPlot(window["PLOT"+id], window["data"+id], window["layout"+id]);
                }}
                else if (type === "PREDRAWXSY") {{
                    var len = parseInt(messages[2], 10);
                    var yVec = new Float32Array(len);
                    var xVec = new Array(len);;
                    for (ind = 0; ind < len; ind++) {{
                        yVec[ind] = parseFloat(messages[3+ind]);
                    }}
                    for (ind = 0; ind < len; ind++) {{
                        xVec[ind] = messages[len+3+ind];
                    }}
                    window["data" + id][0].y = yVec;
                    window["data" + id][0].x = xVec;
                    Plotly.newPlot(window["PLOT"+id], window["data"+id], window["layout"+id]);
                }}
                else if (type === "CLICK") {{
                    var fun = window[id+"_func"];
                    fun();
                    //item.click();
                }}
            }};
            window.onbeforeunload = function(e) {{
                conn_ws.send("CLOSE");
            }};
        
            "#,
            self.ws_url.clone()
        );
        let cbs = Callbacks::default();
        (cbs, e)
    }

    /// id_new generates a unique id
    fn id_new(&self) -> String {
        let mut id_cnt = self.id_cnt.lock().unwrap();
        *id_cnt = *id_cnt + 1;
        let id_str = format!("ID{}", *id_cnt);
        id_str
    }

    /// button_new creates a button, pass the B5 button type,
    /// the button text.
    /// B5 button types are: "primary", "secondary", "success", "danger",
    /// "warning", "info", "light", "dark".
    pub fn button_new(&self, b_type: &str, text: &str) -> Elem {
        let id = self.id_new();
        let h_text = format!(
            r#"
            <button type="button" class="btn btn-{} m-2" id="{}" onclick="{}_func()">
            <span id="{}text">{}</span></button>
            "#,
            b_type, id, id, id, text
        );

        let mut e = Elem::default();
        e.h_start = h_text.clone();
        e.html = h_text;
        e.id = id;
        e.el_type = ElType::ButtonT;
        e.tx_ws = Some(self.tx_ws.clone());

        e.js = format!(
            r#"
        function {}_func() {{
            conn_ws.send("/{}");
        }}
        "#,
            e.id, e.id
        );

        e
    }

    /// button_withicon_new creates a button, pass the B5 type, the icon name, the button text.
    pub fn button_withicon_new(&self, b_type: &str, icon_name: &str, text: &str) -> Elem {
        let id = self.id_new();
        let h_text = format!(
            r#"
        <button type="button" class="btn btn-{} m-2" id="{}" onclick="{}_func()">
        <span class="btn-label">
        <img src="static/bootstrap-icons/{}.svg" alt="" width="16" height="16"></i></span>
        <span id="{}text">{}</span></button>
            "#,
            b_type, id, id, icon_name, id, text
        );

        let mut e = Elem::default();
        e.h_start = h_text.clone();
        e.html = h_text;
        e.id = id;
        e.el_type = ElType::ButtonT;
        e.tx_ws = Some(self.tx_ws.clone());

        e.js = format!(
            r#"
    function {}_func() {{
        conn_ws.send("/{}");
    }}
    "#,
            e.id, e.id
        );

        e
    }

    /// label_new creates a label; pass the label text.
    pub fn label_new(&self, text: &str) -> Elem {
        let id = self.id_new();
        let h_text = format!(
            r#"
            <label class="m-2" id={}>{}</label>
            "#,
            id, text
        );

        let mut e = Elem::default();
        e.h_start = h_text.clone();
        e.html = h_text;
        e.id = id;
        e.el_type = ElType::LabelT;
        e.tx_ws = Some(self.tx_ws.clone());

        return e;
    }

    /// dropdown_new creates a button dropdown; pass the type (same as button),
    /// the button text and the list of options.
    pub fn dropdown_new(&self, b_type: &str, text: &str, list: Vec<&str>) -> Elem {
        let id = self.id_new();

        let mut h_text = format!(
            r#"
        <div class="dropdown">
        <button class="btn btn-{} m-2 dropdown-toggle" role="button" id="{}" data-bs-toggle="dropdown" aria-expanded="false" >
      {}
        </button>
        <ul class="dropdown-menu" id="{}" onclick="{}_func(event)">
        "#,
            b_type, id, text, id, id
        );

        for elem in list {
            h_text = format!(
                r#"{}<li><a class="dropdown-item" href="{}">{}</a></li>
            "#,
                h_text, "#", elem
            );
        }
        h_text = format!(
            r#"{}</ul>
        </div>
        "#,
            h_text
        );

        let mut e = Elem::default();
        e.h_start = h_text.clone();
        e.html = h_text;
        e.id = id;
        e.el_type = ElType::DDownT;
        e.tx_ws = Some(self.tx_ws.clone());

        e.js = format!(
            r#"
        function {}_func(e) {{
            var val = e.target.innerHTML;
            conn_ws.send("/{}@@" + val);
        }}
        "#,
            e.id, e.id
        );

        e
    }

    /// inputtext_new creates a input text field.
    /// Pass in input the placeholder string..
    pub fn inputtext_new(&self, placeholder: &str) -> Elem {
        let id = self.id_new();

        let h_text = format!(
            r#"
        <input type="text" class="m-2" id="{}" name="{}" placeholder="{}" style="text-align: right;" onkeypress="{}_func(event)">
    "#,
            id, id, placeholder, id
        );
        let mut e = Elem::default();
        e.h_start = h_text.clone();
        e.html = h_text;
        e.id = id;
        e.el_type = ElType::ITextT;
        e.tx_ws = Some(self.tx_ws.clone());

        e.js = format!(
            r#"
    function {}_func(e) {{
        if(e.keyCode == 13) {{
            var val = document.getElementById("{}").value;
            conn_ws.send("/{}@@" + val);
        }}
    }}
    "#,
            e.id, e.id, e.id
        );

        e
    }

    /// file_input_new allows to select a file; pass the label text
    pub fn file_input_new(&self, text: &str) -> Elem {
        let id = self.id_new();

        let h_text = format!(
            r#"
            <div class="input-group m-2">
  	<button class="btn btn-outline-secondary" type="button" id="{}" onclick="{}_func()">{}</button>
  	<input type="file" class="form-control" id="{}file" aria-describedby="{}" aria-label="Upload">
    </div>
    "#,
            id, id, text, id, id
        );
        let mut e = Elem::default();
        e.h_start = h_text.clone();
        e.html = h_text;
        e.id = id;
        e.el_type = ElType::ITextT;
        e.tx_ws = Some(self.tx_ws.clone());

        e.js = format!(
            r#"
    function {}_func(e) {{
        var val = document.getElementById("{}file").value;
        conn_ws.send("/{}@@" + val);
    }}
    "#,
            e.id, e.id, e.id
        );

        e
    }

    /// tabs_new creates a nav-tabs;  pass a vector of tab texts;
    /// contained tabs are returned as SubElems.
    pub fn tabs_new(&self, texts: Vec<&str>) -> Elem {
        let mut ids: Vec<String> = vec![];
        let mut elems: Vec<Elem> = vec![];

        for _ in 0..texts.len() {
            ids.push(self.id_new());
        }

        let mut h_text = format!(
            r#"
        <ul class="nav nav-tabs">
"#
        );
        for (ind, id) in ids.iter().enumerate() {
            let mut link_type: &str = "";
            if ind == 0 {
                link_type = "active";
            }
            h_text = format!(
                r#"{}
            <li class="nav-item" role="presentation">
            <a class="nav-link {}" data-bs-toggle="tab" data-bs-target="{}{}" id="t_{}">{}</a>
        </li>
    "#,
                h_text, link_type, '#', id, id, texts[ind]
            );
        }
        h_text = format!(
            r#"{}
        </ul>
        "#,
            h_text
        );

        let mut tabs = Elem::default();
        tabs.h_start = h_text.clone();
        tabs.html = h_text;
        tabs.id = "tabs".to_string();
        tabs.el_type = ElType::TabsT;
        tabs.tx_ws = Some(self.tx_ws.clone());

        for (ind, id) in ids.iter().enumerate() {
            let pane_type: &str;
            if ind == 0 {
                pane_type = "show active";
            } else {
                pane_type = "fade";
            }
            let h_start = format!(
                r#"
            <div class="tab-pane {}" id="{}">
            "#,
                pane_type, id
            );
            let h_end = format!("</div>");
            let mut e = Elem::default();
            e.h_start = h_start.clone();
            e.h_end = h_end;
            e.html = h_start;
            e.id = id.to_string();
            e.el_type = ElType::TPaneT;
            e.tx_ws = Some(self.tx_ws.clone());
            elems.push(e);
        }
        tabs.sub_elems = elems;
        tabs.sub_start = format!(r#"<div class="tab-content">"#);
        tabs.sub_end = format!("</div>");

        tabs
    }

    /// modal_new creates a Modal Dialog; pass the dialog
    /// title and general text, button texts.
    pub fn modal_new(&self, title: &str, text: &str, bt1_text: &str, bt2_text: &str) -> Elem {
        let id1 = self.id_new();
        let id2 = self.id_new();
        let h_start = format!(
            r#"
            <div class="modal" tabindex="-1" id="{}{}">
            <div class="modal-dialog">
              <div class="modal-content">
                <div class="modal-header">
                  <h5 class="modal-title">{}</h5>
                </div>
                <div class="modal-body">
                  <p>{}</p>
                </div>
                <div class="modal-footer">
                  <button type="button" class="btn btn-primary" data-bs-dismiss="modal" onclick="{}_func()">{}</button>
                  <button type="button" class="btn btn-secondary" data-bs-dismiss="modal" onclick="{}_func()">{}</button>
                </div>
              </div>
            </div>
            </div>
            "#,
            id1, id2, title, text, id1, bt1_text, id2, bt2_text
        );
        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.html = h_start;
        e.id = format!("{}{}", id1, id2);
        e.el_type = ElType::ModalT;
        e.tx_ws = Some(self.tx_ws.clone());

        let (tx_modal, rx_modal): (Sender<bool>, Receiver<bool>) = crossbeam_channel::unbounded();
        e.tx_modal = Some(tx_modal);
        e.rx_modal = Some(rx_modal);

        let mut e1 = Elem::default();
        e1.id = id1.clone();
        e1.el_type = ElType::ButtonT;

        let mut e2 = Elem::default();
        e2.id = id2.clone();
        e2.el_type = ElType::ButtonT;

        e.sub_elems = vec![e1, e2];
        e.js = format!(
            r#"
        function {}_func() {{
            conn_ws.send("/{}");
        }}

        function {}_func() {{
            conn_ws.send("/{}");
        }}

        "#,
            &id1, &id1, &id2, &id2
        );

        e
    }

    /// TextAreaNew creates a textarea; the number of rows.
    /// Remember to attach a callback to handle output om the area.
    pub fn textarea_new(&self, rows: i32) -> Elem {
        let id = self.id_new();

        let h_text = format!(
            r#"
        <div class="form-group mx-2" style="min-width: 90%">
        <p><textarea class="form-control" id={} rows="{}"></textarea></p>
        </div>
"#,
            id, rows
        );

        let ta_ws_listener = TcpListener::bind("127.0.0.1:0").expect("tcp server error");
        let addr = ta_ws_listener.local_addr().unwrap().clone();
        let (tx_ws, rx_ws): (Sender<String>, Receiver<String>) = crossbeam_channel::unbounded();

        spawn(move || {
            for stream in ta_ws_listener.incoming() {
                let stream = stream.unwrap();

                let mut websocket = accept(stream).unwrap();

                loop {
                    select! {
                        recv(rx_ws) -> rx_msg =>  {
                            websocket.send(Message::Text(rx_msg.unwrap())).unwrap();
                        }
                    }
                }
            }
        });

        let mut e = Elem::default();
        e.h_start = h_text.clone();
        e.html = h_text;
        e.id = id;
        e.el_type = ElType::TextAreaT;
        e.tx_ws = Some(self.tx_ws.clone());
        e.tx_ws_for_ta = Some(tx_ws);

        e.js = format!(
            r#"
        var text = document.getElementById("{}");
        var addr1 = "ws://" + "{}";
        conn_{} = new WebSocket(addr1);
        conn_{}.onmessage = function (evt) {{
            var edata = evt.data;
            var messages = edata.split('\n');
            for (var i = 0; i < messages.length; i++) {{
                if (messages[i] != "") {{
                    var str = messages[i];
                    str = text.value + str;
                    diff = str.length - 4096;
                    if (diff > 0) {{
                        text.value = str.slice(diff) + '\n';
                    }} else {{
                        text.value = str + '\n';
                    }}
                }}
            }}
            if ((messages.length == 1) && (messages[0] == '')){{
                text.value = "";
            }}
            text.scrollTop = text.scrollHeight;
        }};    
    "#,
            &e.id, &addr, &e.id, &e.id
        );

        e
    }

    /// container_new creates a container.
    pub fn container_new(&self) -> Elem {
        let id = self.id_new();

        let h_start = format!(
            r#"
            <div class="container" id="{}">
    "#,
            id
        );
        let h_end = format!(
            r#"
            </div>
    "#
        );
        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.html = h_start;
        e.h_end = h_end;
        e.id = id;
        e.el_type = ElType::ContainerT;
        e.tx_ws = Some(self.tx_ws.clone());

        e
    }

    /// row_new creates a row.
    pub fn row_new(&self) -> Elem {
        let id = self.id_new();

        let h_start = format!(
            r#"
            <div class="row" id="{}">
    "#,
            id
        );
        let h_end = format!(
            r#"
            </div>
    "#
        );
        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.html = h_start;
        e.h_end = h_end;
        e.id = id;
        e.el_type = ElType::RowT;
        e.tx_ws = Some(self.tx_ws.clone());

        e
    }

    /// col_new creates a col.
    pub fn col_new(&self) -> Elem {
        let id = self.id_new();

        let h_start = format!(
            r#"
            <div class="col align-self-center" id="{}">
    "#,
            id
        );
        let h_end = format!(
            r#"
            </div>
    "#
        );
        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.html = h_start;
        e.h_end = h_end;
        e.id = id;
        e.el_type = ElType::ColT;
        e.tx_ws = Some(self.tx_ws.clone());

        e
    }

        // colspan_new creates a col with fixed width, pass the span (1, 2, 4, 6, 12).
        pub fn colspan_new(&self, span: i32) -> Elem {
            let id = self.id_new();
    
            let h_start = format!(
                r#"
                <div class="col-{} align-self-center" id="{}">
        "#,
                span, id
            );
            let h_end = format!(
                r#"
                </div>
        "#
            );
            let mut e = Elem::default();
            e.h_start = h_start.clone();
            e.html = h_start;
            e.h_end = h_end;
            e.id = id;
            e.el_type = ElType::ColT;
            e.tx_ws = Some(self.tx_ws.clone());
    
            e
        }
    
    /// image_new creates an image tag; pass  the name of the image file,
    /// the size. The image file must reside in /static
    pub fn image_new(&self, file_name: &str, width: i32, height: i32) -> Elem {
        let id = self.id_new();

        let h_start = format!(
            r#"
            <img id="{}" src="static/{}" alt="missing img" width="{}" height="{}">
    "#,
            id, file_name, width, height
        );

        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.html = h_start;
        e.id = id;
        e.el_type = ElType::ImageT;
        e.tx_ws = Some(self.tx_ws.clone());

        e
    }

    /// card_new creates a card; pass header and title text.
    pub fn card_new(&self, header: &str, title: &str) -> Elem {
        let id = self.id_new();

        let h_start = format!(
            r#"
            <div class="card">
            <h5 class="card-header">{}</h5>
            <div class="card-body" id="{}">
            <h5 class="card-title">{}</h5>
    "#,
            header, id, title
        );
        let h_end = format!(
            r#"
            </div>
            </div>
    "#
        );
        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.html = h_start;
        e.h_end = h_end;
        e.id = id;
        e.el_type = ElType::CardT;
        e.tx_ws = Some(self.tx_ws.clone());

        e
    }

    pub fn radio_new(&self, text: &Vec<&str>, checked_ind: i32) -> Elem {
        let mut ids: Vec<String> = vec![];
        for _elem in text.clone() {
            ids.push(self.id_new());
        }

        let mut h_start = String::default();
        let mut checked: String;
        let single_id = format!("{}{}", ids[0], "radio");

        for (ind, id) in ids.iter().enumerate() {
            if ind == checked_ind as usize {
                checked = "checked".to_string();
            } else {
                checked = "".to_string();
            }

            h_start = format!(
                r#"
                {}
                <div class="form-check">
                <input class="form-check-input" type="radio" value="{}" name="{}" id="{}" {} onclick="{}_func(event)">
                <label class="form-check-label" for="{}">
                {}
                </label>
                  </div>
            "#,
                h_start, ind, single_id, id, checked, single_id, id, text[ind]
            );
    
        }
        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.html = h_start;
        e.id = single_id.clone();
        e.el_type = ElType::RadioT;
        e.tx_ws = Some(self.tx_ws.clone());

        e.js = format!(
            r#"
        function {}_func(e) {{
            var val = e.target.value;
            conn_ws.send("/{}@@" + val);
        }}

        "#,
            &single_id, &single_id
        );

        e
    }

    /// rangeslider_new creates a slider, pass initial, min, max, step values.
    pub fn rangeslider_new(&self, initial: f32, min: f32, max: f32, step: f32) -> Elem {
        let id = self.id_new();

        let h_start = format!(
            r#"
            <input id="{}" type="range" class="form-range" min="{}" max="{}" step="{}" onchange="{}_func()">
    "#,
            id, min, max, step, id
        );
        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.html = h_start;
        e.id = id.clone();
        e.el_type = ElType::RSliderT;
        e.tx_ws = Some(self.tx_ws.clone());

        e.js = format!(
            r#"
            function {}_func() {{
                var val = document.getElementById("{}").value; 
                conn_ws.send("/{}@@" + val);
            }}
            document.getElementById("{}").value = "{}"; 
    
        "#,
            id, id, id, id, initial
        );

        e
    }

    /// pillbadge_new creates a pill badge, pass the
    /// type (same as Button), the text.
    pub fn pillbadge_new(&self, b_type: &str, text: &str) -> Elem {
        let id = self.id_new();

        let h_start = format!(
            r#"
            <span id="{}" class="badge rounded-pill bg-{}">{}</span>
    "#,
            id, b_type, text
        );
        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.html = h_start;
        e.id = id.clone();
        e.el_type = ElType::PBadgeT;
        e.tx_ws = Some(self.tx_ws.clone());

        e
    }

    /// paragraph_new creates a paragraph.
    pub fn paragraph_new(&self) -> Elem {
        let id = self.id_new();

        let h_start = format!(
            r#"
            <p id="{}">
    "#,
            id
        );
        let h_end = format!("</p>");

        let mut e = Elem::default();
        e.h_start = h_start.clone();
        e.h_end = h_end;
        e.html = h_start;
        e.id = id.clone();
        e.el_type = ElType::ParagraphT;
        e.tx_ws = Some(self.tx_ws.clone());

        e
    }

    /// grid_new creates a container with a row col grid; content is input
    /// through an array of ColsSpans (one array element for every row).
    /// Use paragraph for empty column.
    /// Use 0 for no span.
    pub fn grid_new(&self, col_spans: Vec<ColSpans>) -> Elem {
        let mut ct = self.container_new();
        for col_span in col_spans {
            let mut row = self.row_new();
            for (ind, elem) in col_span.elems.iter().enumerate() {
                let mut col: Elem;
                let span = col_span.spans[ind];
                if span != 0 {
                    col = self.colspan_new(span);
                } else {
                    col = self.col_new();
                }
                col.add(elem);
                row.add(&col);
            }
            ct.add(&row);
        }

        ct
    }


}

#[macro_export]
macro_rules! exec_if_modal_selected {
    ($i:expr, $e:expr) => {
        std::thread::spawn(move || {
            if $i.modal_selected() {
                $e;
            }
        });
    };
}
