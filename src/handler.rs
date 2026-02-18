use hickory_server::authority::MessageResponseBuilder;
use hickory_server::server::{Request, RequestHandler, ResponseInfo};
use std::sync::Arc;

use crate::AppState;

pub type SharedState = Arc<AppState>;
pub struct DnsHandler {
    pub state: SharedState,
}

impl RequestHandler for DnsHandler {
    async fn handle_request(&self,request: Request,response_handle: R) -> ResponseInfo {
        // implement dns blocking logic here
        let builder = MessageResponseBuilder::from_message_request(MessageRequest);
        let response = builder.build_no_records(*request.header());
        response_handle.send_response(response).await.unwrap_or_else(|e| {
            eprintln!("Error sending response: {}", e);
            request.header().into()
        })
        todo!();
    }
}