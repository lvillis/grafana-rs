use crate::{
    BlockingClient, Result,
    types::{
        AddTeamMemberRequest, CreateTeamRequest, CreateTeamResponse, SuccessResponse, Team, TeamId,
        TeamMember, TeamSearchParams, TeamSearchResult, UpdateTeamMemberRequest, UpdateTeamRequest,
        UserId,
    },
};

#[derive(Clone)]
pub struct BlockingTeamsService {
    client: BlockingClient,
}

impl BlockingTeamsService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn search(&self, params: &TeamSearchParams) -> Result<TeamSearchResult> {
        self.client.get_json(&["teams", "search"], Some(params))
    }

    pub fn create(&self, request: &CreateTeamRequest) -> Result<CreateTeamResponse> {
        self.client.post_json(&["teams"], request)
    }

    pub fn get_by_id(&self, team_id: impl Into<TeamId>) -> Result<Team> {
        let team_id: TeamId = team_id.into();
        let team_id_str = team_id.0.to_string();
        let segments = ["teams", team_id_str.as_str()];
        self.client.get_json(&segments, Option::<&()>::None)
    }

    pub fn update(
        &self,
        team_id: impl Into<TeamId>,
        request: &UpdateTeamRequest,
    ) -> Result<SuccessResponse> {
        let team_id: TeamId = team_id.into();
        let team_id_str = team_id.0.to_string();
        let segments = ["teams", team_id_str.as_str()];
        self.client.put_json(&segments, request)
    }

    pub fn delete(&self, team_id: impl Into<TeamId>) -> Result<SuccessResponse> {
        let team_id: TeamId = team_id.into();
        let team_id_str = team_id.0.to_string();
        let segments = ["teams", team_id_str.as_str()];
        self.client.delete_json(&segments)
    }

    pub fn members(&self, team_id: impl Into<TeamId>) -> Result<Vec<TeamMember>> {
        let team_id: TeamId = team_id.into();
        let team_id_str = team_id.0.to_string();
        let segments = ["teams", team_id_str.as_str(), "members"];
        self.client.get_json(&segments, Option::<&()>::None)
    }

    pub fn add_member(
        &self,
        team_id: impl Into<TeamId>,
        user_id: impl Into<UserId>,
    ) -> Result<SuccessResponse> {
        let user_id: UserId = user_id.into();
        let request = AddTeamMemberRequest::new(user_id.0);
        let team_id: TeamId = team_id.into();
        let team_id_str = team_id.0.to_string();
        let segments = ["teams", team_id_str.as_str(), "members"];
        self.client.post_json(&segments, &request)
    }

    pub fn update_member_permission(
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
        self.client.put_json(&segments, request)
    }

    pub fn remove_member(
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
        self.client.delete_json(&segments)
    }
}
