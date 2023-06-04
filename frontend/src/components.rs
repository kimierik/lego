
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
    let url="/api/redirdoc?name=".to_string() +&data;
    let win =web_sys::window().expect("no window found"); //get win so we can use the js api's.
                                                          //probably does not need to be fetched in
                                                          //every element

    view!{ cx, 
        <div class="element center">

            <div class="elemContent ">
                <p>
                    "Document: " {data} 
                </p>

                <button 
                    on:click=move|_|{
                        win.open_with_url(&url).unwrap(); //currently using {window.open(url)} js command
                                                          //for downloading from the server
                    } >
                "Download file"
                </button>

            </div>
        </div>
    }
}


use crate::style::theme::Themes;

#[component]
fn ThemeSwitcher(cx:Scope, set_theme:WriteSignal<Themes>)->impl IntoView{

    view! {cx,

        <button
            on:click =move |_| {
                set_theme.set( Themes::One);
            }
            >
            "set to one"
        </button>
        <button
            on:click =move |_| {
                set_theme.set( Themes::Two);
            }
            >
            "set to two"
        </button>
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

    //create signals for themes
    let (theme,set_theme)=create_signal(cx, crate::style::theme::Themes::One);
    //style val is a string with css in it
    let (style_val,set_style_val)=create_signal(cx, "".to_string());

    let style_vall= crate::style::get_style(theme);
    set_style_val(style_vall);
    
    create_effect(cx, move |_|{
        set_style_val(crate::style::get_style(theme));
    });

    //so that other components can use it if ever needed
    provide_context(cx, theme);

    view! { cx, 
        <style> {style_val}  </style>
        <div class="Background">
            <div class="AppContainer">
            <ThemeSwitcher set_theme/>
                {theme}
                <h1 class="center">"Documents"</h1>
                {data_view}
            </div>
        </div>
    }

}
