use leptos::*;
use serde::{Deserialize,Serialize};



//data from files
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct RemoteFile{
    name:String,
}


#[component]
fn Element(cx:Scope,data:String)->impl IntoView{

    let url="http://localhost:3000/api/redirdoc?name=".to_string() +&data;

    view!{cx,
        <div>
            <br/>
            <a href={url}>
                    {data}
            </a>
            <br/>
        </div>
    }
}



pub async fn sendreq()-> Result<reqwasm::http::Response, reqwasm::Error>{

    let qres= reqwasm::http::Request::get("http://localhost:3000/api/getfiles").send();

    let thing=qres.await;
    log::info!(" sendreqfn end");
    thing
}

pub async fn reqwrap()->Vec<String>{
    let thing= match sendreq().await {
        Ok(a)=>a,
        Err(e)=>{
            log::info!("error in reqwrap {}",e);
            panic!("{}",e);
        },
    };

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

    let once = create_resource(cx, || (), |_| async move { reqwrap().await });
    view! { cx,
        <h1>"server data"</h1>
            //see if there is data in the data
        {move || match once.read(cx) {
            None => view! { cx, <p>"Loading..."</p> }.into_view(cx),
            Some(data) => data.into_iter().map(|item|{ view! {cx, <Element data=item/>} }).collect_view(cx)
                          //since data is a vec we will iterate it and make views out of them
        }}
    }

}
