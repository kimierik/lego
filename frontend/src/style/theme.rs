
use super::ColorPrio;

pub struct Theme{
    primary:String,
    secondary:String,
    trinary:String,
    tertiary:String,
}

use ColorPrio::*;


#[derive(Clone,Debug)]
pub enum Themes{
    One,
    Two,
}


impl Theme{

    pub fn get_col(&self,selector:ColorPrio)->String{
        match selector {
            Primary=> &self.primary,
            Secondary=>&self.secondary,
            Trinary=>&self.trinary,
            Tertiary=>&self.tertiary,
        }.to_string()

    }



    pub fn get_theme(theme:&Themes)->Self{
        //rn only one theme
        match theme {
            Themes::One=> Theme{
                primary:"#2D2727".to_string(),
                secondary:"#413543".to_string(),
                trinary:"#8F43EE".to_string(),
                tertiary:"#f0EB8D".to_string(),
            },
            Themes::Two=> Theme{
                primary:"#0079FF".to_string(),
                secondary:"#00DFA2".to_string(),
                trinary:"#F6FA70".to_string(),
                tertiary:"#FF0060".to_string(),
            },
            
        }
    }


}


