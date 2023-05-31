use leptos::*;


pub async fn sendreq()-> Result<reqwasm::http::Response, reqwasm::Error>{

    let qres= reqwasm::http::Request::get("http://localhost:3000/api/getfiles").send();

    let thing=qres.await;
    log::info!(" sendreqfn end");
    thing
}

pub async fn reqwrap()->String{
    let thing= match sendreq().await {
        Ok(a)=>a,
        Err(e)=>{
            log::info!("error in reqwrap {}",e);
            panic!("{}",e);
        },
    };
    thing.text().await.unwrap()
}

#[component]
pub fn App(cx: Scope)->impl IntoView{

    let once = create_resource(cx, || (), |_| async move { reqwrap().await });
    view! { cx,
        <h1>"server data"</h1>
        {move || match once.read(cx) {
            None => view! { cx, <p>"Loading..."</p> }.into_view(cx),
            Some(data) => view! { cx,<p>{data}</p>  }.into_view(cx)
        }}
    }

}
