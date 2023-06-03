
pub mod theme;
use theme::Theme;
use leptos::*;


//#f0EB8D;
pub fn get_style( style:ReadSignal<theme::Themes>)->(String,String){
    let thing:theme::Themes=style.get();

    let primary=get_color(ColorPrio::Primary,&thing);
    let secondary=get_color(ColorPrio::Secondary,&thing);
    //let trinary=get_color(ColorPrio::Trinary,&thing);
    let tertiary=get_color(ColorPrio::Tertiary,&thing);

    let mem=format!("

        .element{{
            background-color: {tertiary};
            display: flex;
            justify-content: center;
        }}

        .elemContent{{
            margin: 5px;
            width: 80%;

            background-color: {secondary};
            display:inline-block;
            justify-content: center;
            align-items: center;
            align-content: center;

        }}

        .center{{
            align-items: center;
            align-content: center;
            display: flex;
            justify-content: center;
        }}

        .Background{{
            margin: 0;
            padding: 0;

            background-color: {primary};
            width: 100%;
            height: 100%;
        }}

        .AppContainer{{
            color: white;
            width: 50%;
            margin:  auto;
        }}
        ");

    //let (a,b)=stylers::style_str!(raw_str(&mem));
    ("name".to_string(),mem)
}



pub enum ColorPrio{
    Primary,
    Secondary,
    Trinary,
    Tertiary,
}



fn get_color(prio:ColorPrio,theme:&theme::Themes)->String{
    //this function needs to know the theme
    Theme::get_theme(theme).get_col(prio)
}










