use leptos::*;
use crate::utils::get_file_request;


/**
  element that represents files on the bakcend
  has a button that can be used to download the file
  data= the file name
  */
#[component]
fn Element(cx:Scope,data:String)->impl IntoView{

    // this should not be a <a> with href it should open the html ontop or download or somethign
    let url="http://localhost:3000/api/redirdoc?name=".to_string() +&data;
    let win =web_sys::window().expect("no window found"); //get win so we can use the js api's.
                                                          //probably does not need to be fetched in
                                                          //every element

    view!{ cx, 
        <div >
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




/*
 * the main app page
 * */
#[component]
pub fn App(cx: Scope)->impl IntoView{
    // resource for the file names
    let once = create_resource(cx, || (), |_| async move { get_file_request().await });

    // elements represented as views
    let data_view= move || match once.read(cx) {
        None => view! { cx, <p>"Loading..."</p> }.into_view(cx),
        Some(data) => data.into_iter().map(|item|{ view! {cx, <Element data=item/>} }).collect_view(cx)
              //since data is a vec we will iterate it and make views out of them
    };

    view! { cx,
        <h1>"Documents"</h1>
        <div>
            {data_view}
        </div>
    }

}
