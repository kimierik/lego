
use serde::{Deserialize,Serialize};

/**
  sends get request to backend/api
 
  */
pub async fn send_get_request(path:String)-> reqwasm::http::Response {

    //http://localhost:3000/api/
    let qres= reqwasm::http::Request::get(&("api/".to_string()+&path)).send().await;
    match qres {
        Ok(a)=>a,
        Err(e)=>{
            log::info!("error in reqwrap {}",e);
            panic!("{}",e);//panic for now. TODO change
        },
    }

}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct RemoteFile{
    name:String,
}

/**
  requests file names and returns vec of strings containing the names
  */
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


