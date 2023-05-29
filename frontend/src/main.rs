use leptos::{*, html::Output};
use leptos_router::*;


//#[component]
//pub fn Elem(cx:Scope, name:string)->impl IntoView{}


pub async fn get_g_data()->String{

    log::info!(" gdata");
    //let abort_controller = web_sys::AbortController::new().ok();
    //let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    //server does not get the req
    let json = gloo_net::http::Request::get("http://localhost:3000/api/getfiles").header("Access-Control-Allow-Origin:", "*").send()  ; //.send().await;

    log::info!(" post reg");
    let a=json.await;
    log::info!(" post await");
    let c=a.unwrap();
    let b=c.text().await.unwrap();
    b
}

// this does not trigger cors but does not work
async fn sendreq()-> Result<reqwasm::http::Response, reqwasm::Error>{
    log::info!(" sendreqfn start");

    /*
     *
    let qres = reqwest::Client::new()
            .get("http://localhost:3000/api/getfiles")
            .fetch_mode_no_cors()
            .send();
     */
    //this is the same shit. server gets but something fucks// i quess does not send to server anymore
    let qres= reqwasm::http::Request::get("http://localhost:3000/api/getfiles").send();
    //let qres = reqwest::blocking::get("http://localhost:3000/api/getfiles");

    log::info!(" starting to wait");
    let thing=qres.await;//blocks here server gets req
    log::info!(" sendreqfn end");
    thing
}

pub async fn get_data_c()->String{
    log::info!(" con req start");

    let asd=sendreq();

    log::info!("asdf");
    //let qres=asd.await;
    let qres=block_on(asd); //what? this does not even start the log on the start of the function

    log::info!("req constructer");//does not get to here this blocks forwver
    //let res = block_on(qres).unwrap(); //this does not end up sending the rex

    let res = match qres{
        Ok(i) => {i},
        Err(e) => {
            log::info!("res error {} ",e);
            panic!("sa");
        },
    }; //this sends the actual req
    //is unwrap problem???? does not seem like it
    log::info!("res made");


    /*
    let res = reqwest::Client::new()
            .get("http://localhost:3000/api/getfiles")
            .send()
            .await.unwrap();
      */

    let text = res.text().await.unwrap();
    log::info!("req trext");
    text
}

/*
 //this triggers cors
pub async fn get_data()->String{
    let json = reqwest::get("http://localhost:3000/api/getfiles").await;
    let a=json.unwrap();
    let b=a.text().await.unwrap();
    b
}
 * */

/*
pub async fn set_string(stthi:WriteSignal<String> ){
    let thing=get_data().await;
    stthi.set(thing);
}

pub async fn set_debuf(s:&mut String){
    log::info!(" debuf");
    *s=get_data().await;
}

 * */
async fn something(){
    log::info!(" log");
}
async fn re_something()->String{
    "something".to_string()
}

async fn something_else(){
    log::info!(" another log");
    something().await;

    log::info!(" another {}",re_something().await );
}

async fn reqwrap(){
    //this probably crashes since we cant do 2 block
    // i love that it tells me this
    let thing=sendreq().await.unwrap();
    log::info!("{}", thing.text().await.unwrap())
}


use futures::{executor::block_on, Future};

#[component]
pub fn App(cx: Scope)->impl IntoView{
    // get the data
    let (data,set_data)=create_signal(cx, "i".to_string());

    let mut test="a".to_string();
    log::info!("app mount");

    //set_debuf(&mut test);//also does not call
    something();
    let s=something_else();

    futures::executor::block_on(s);
    //set_string(set_data);

    // does not updatee so i prob need the signal thing

    view! {cx,
        <div>
        <p>
            {data}
            "\nthis is from the page that is front asdflkj"
        </p>
           <button on:click=move |_|{
                log::info!("bdn pres");
                 //let dat= block_on(get_g_data());
                let dat= block_on(reqwrap());

                //set_data.set(dat);
           }>
           "button click"
           </button>
            <a href="http://localhost:3000/api/redirdoc?name=thing.html">"click"</a>
        </div>

    }
}




pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|cx| view! { cx, <App/>  })
}
