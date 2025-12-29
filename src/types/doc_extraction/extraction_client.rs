use reqwest::{Body, StatusCode, multipart::{Form,Part}};
use std::path::Path;
use tokio_util::io::ReaderStream;

use crate::{
    enums::SDKError,
    traits::ToOpenedResponse,
    types::{NewScanSession,ApiSuccess,Client},
};

type Result<T> = std::result::Result<T,SDKError>;

#[derive(Clone,Copy,Debug)]
pub struct DocExtractionClient<'a> {
    client: &'a Client
}

impl <'a> DocExtractionClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create_scan_session(&self) -> Result<ApiSuccess<NewScanSession>> {
        // build base query
        let base_req = self.client.post("/v1/extractions/sessions")?;

        // append auth headers when available
        let req = self.client
            .auth_req(base_req)
            .build()?;

        // execute the query and format the response
        let response = self.client
            .http()
            .execute(req)
            .await
            .map_err(SDKError::from_reqwest)?
            .open()
            .await?;

        Ok(response)
    }

    pub async fn upload_single_scan(&self,session_id: i64,file_path: &Path,file_name: &str,byte_size: u64, mime_str:&str) -> Result<()> {
        // configure the client
        let base_req = self.client.post(&format!("/v1/extractions/sessions/{session_id}/single"))?;

        // verify correct file size
        let actual = tokio::fs::metadata(file_path).await?.len();
        
        if actual != byte_size {
            return Err(SDKError::FileSizeMismatch { expected: byte_size, actual });
        }

        // === prepare the upload parts === //

        // stream
        let file = tokio::fs::File::open(file_path).await?;
        let file_name = file_name.to_owned();
        let stream = ReaderStream::new(file);
        let body = Body::wrap_stream(stream);

        // part
        let part = Part::stream_with_length(body, byte_size)
            .file_name(file_name.clone())
            .mime_str(mime_str)?;

        // form
        let form = Form::new()
            .text("file_name", file_name)
            .text("byte_size", byte_size.to_string())
            .part("file", part);

        // build the request
        let req = self.client
            .auth_req(base_req)
            .multipart(form)
            .build()?;

        // send + process response
        let response = self.client
            .http()
            .execute(req)
            .await
            .map_err(SDKError::from_reqwest)?;

        match response.status() {
            StatusCode::CREATED     => Ok(()),
            StatusCode::ACCEPTED    => Ok(()),
            _ => Err(SDKError::UnexpectedResponseType(response.status()))
        }
    }
}