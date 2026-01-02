use crate::{
    Client, Result,
    types::{
        AddTeamMemberRequest, CreateTeamRequest, CreateTeamResponse, SuccessResponse, Team, TeamId,
        TeamMember, TeamSearchParams, TeamSearchResult, UpdateTeamMemberRequest, UpdateTeamRequest,
        UserId,
    },
};

#[derive(Clone)]
pub struct TeamsService {
    client: Client,
}

impl TeamsService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn search(&self, params: &TeamSearchParams) -> Result<TeamSearchResult> {
        self.client
            .get_json(&["teams", "search"], Some(params))
            .await
    }

    pub async fn create(&self, request: &CreateTeamRequest) -> Result<CreateTeamResponse> {
        self.client.post_json(&["teams"], request).await
    }

    pub async fn get_by_id(&self, team_id: impl Into<TeamId>) -> Result<Team> {
        let team_id: TeamId = team_id.into();
        let team_id_str = team_id.0.to_string();
        let segments = ["teams", team_id_str.as_str()];
        self.client.get_json(&segments, Option::<&()>::None).await
    }

    pub async fn update(
        &self,
        team_id: impl Into<TeamId>,
        request: &UpdateTeamRequest,
    ) -> Result<SuccessResponse> {
        let team_id: TeamId = team_id.into();
        let team_id_str = team_id.0.to_string();
        let segments = ["teams", team_id_str.as_str()];
        self.client.put_json(&segments, request).await
    }

    pub async fn delete(&self, team_id: impl Into<TeamId>) -> Result<SuccessResponse> {
        let team_id: TeamId = team_id.into();
        let team_id_str = team_id.0.to_string();
        let segments = ["teams", team_id_str.as_str()];
        self.client.delete_json(&segments).await
    }

    pub async fn members(&self, team_id: impl Into<TeamId>) -> Result<Vec<TeamMember>> {
        let team_id: TeamId = team_id.into();
        let team_id_str = team_id.0.to_string();
        let segments = ["teams", team_id_str.as_str(), "members"];
        self.client.get_json(&segments, Option::<&()>::None).await
    }

    pub async fn add_member(
        &self,
        team_id: impl Into<TeamId>,
        user_id: impl Into<UserId>,
    ) -> Result<SuccessResponse> {
        let user_id: UserId = user_id.into();
        let request = AddTeamMemberRequest::new(user_id.0);
        let team_id: TeamId = team_id.into();
        let team_id_str = team_id.0.to_string();
        let segments = ["teams", team_id_str.as_str(), "members"];
        self.client.post_json(&segments, &request).await
    }

    pub async fn update_member_permission(
        &self,
        team_id: impl Into<TeamId>,
        user_id: impl Into<UserId>,
        request: &UpdateTeamMemberRequest,
    ) -> Result<SuccessResponse> {
        let team_id: TeamId = team_id.into();
        let user_id: UserId = user_id.into();
        let team_id_str = team_id.0.to_string();
        let user_id_str = user_id.0.to_string();
        let segments = [
            "teams",
            team_id_str.as_str(),
            "members",
            user_id_str.as_str(),
        ];
        self.client.put_json(&segments, request).await
    }

    pub async fn remove_member(
        &self,
        team_id: impl Into<TeamId>,
        user_id: impl Into<UserId>,
    ) -> Result<SuccessResponse> {
        let team_id: TeamId = team_id.into();
        let user_id: UserId = user_id.into();
        let team_id_str = team_id.0.to_string();
        let user_id_str = user_id.0.to_string();
        let segments = [
            "teams",
            team_id_str.as_str(),
            "members",
            user_id_str.as_str(),
        ];
        self.client.delete_json(&segments).await
    }
}
