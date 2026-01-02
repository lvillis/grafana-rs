use crate::{
    Client, Result,
    types::{CreateFolderRequest, DeleteFolderResponse, Folder, FolderUid, UpdateFolderRequest},
};

#[derive(Clone)]
pub struct FoldersService {
    client: Client,
}

impl FoldersService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<Folder>> {
        self.client
            .get_json(&["folders"], Option::<&()>::None)
            .await
    }

    pub async fn get_by_uid(&self, uid: impl Into<FolderUid>) -> Result<Folder> {
        let uid: FolderUid = uid.into();
        let segments = ["folders", uid.0.as_str()];
        self.client.get_json(&segments, Option::<&()>::None).await
    }

    pub async fn create(&self, request: &CreateFolderRequest) -> Result<Folder> {
        self.client.post_json(&["folders"], request).await
    }

    pub async fn update(
        &self,
        uid: impl Into<FolderUid>,
        request: &UpdateFolderRequest,
    ) -> Result<Folder> {
        let uid: FolderUid = uid.into();
        let segments = ["folders", uid.0.as_str()];
        self.client.put_json(&segments, request).await
    }

    pub async fn delete_by_uid(&self, uid: impl Into<FolderUid>) -> Result<DeleteFolderResponse> {
        let uid: FolderUid = uid.into();
        let segments = ["folders", uid.0.as_str()];
        self.client.delete_json(&segments).await
    }
}
