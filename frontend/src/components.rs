use leptos::*;
use serde::{Deserialize,Serialize};



//data from files
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct RemoteFile{
    name:String,
}

/*
            <br/>
            <a href={url}> {data} </a>//change this i want it to be a normal rq not a href
            <br/>
*/



#[component]
fn Element(cx:Scope,data:String)->impl IntoView{

    // this should not be a <a> with href it should open the html ontop or download or somethign
    let url="http://localhost:3000/api/redirdoc?name=".to_string() +&data;
    let win =web_sys::window().expect("no win"); //get win so we can use the js api's

    view!{cx,
        <div>
        <button 
            on:click=move|_|{
                win.open_with_url(&url).unwrap(); //currently using {window.open(url)} js command
                                                  //for downloading from the server
            } >
            {data}
        </button>
        </div>
    }
}



pub async fn send_get_request(path:String)-> reqwasm::http::Response {

    let qres= reqwasm::http::Request::get(&("http://localhost:3000/api/".to_string()+&path)).send().await;
    match qres {
        Ok(a)=>a,
        Err(e)=>{
            log::info!("error in reqwrap {}",e);
            panic!("{}",e);//panic for now. TODO change
        },
    }

}

pub async fn get_file_request()->Vec<String>{
    let thing= send_get_request("getfiles".to_string()).await;

    // turn to json
    thing.json::<Vec<RemoteFile>>()
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<_>>()
}



#[component]
pub fn App(cx: Scope)->impl IntoView{

    let once = create_resource(cx, || (), |_| async move { get_file_request().await });

    let data_view= move || match once.read(cx) {
        None => view! { cx, <p>"Loading..."</p> }.into_view(cx),
        Some(data) => data.into_iter().map(|item|{ view! {cx, <Element data=item/>} }).collect_view(cx)
              //since data is a vec we will iterate it and make views out of them
    };

    view! { cx,
        <h1>"server data"</h1>
        {data_view}
    }

}
