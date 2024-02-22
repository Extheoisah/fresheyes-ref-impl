use fresh_eyes::{
    extract_pr_details, get_pull_request_reviews, Branch, ForkRequest as LibForkRequest,
    PullRequest as LibPullRequest,
};
use fresheyes::git_hub_service_server::{GitHubService, GitHubServiceServer};
use fresheyes::{ForkRequest, ForkResult, PullRequest, PullRequestDetails}; // Import necessary functions
use tonic::{transport::Server, Request, Response, Status};

pub mod fresheyes {
    tonic::include_proto!("fresheyes");
}

#[derive(Debug, Default)]
pub struct GitHubServiceImpl {}

#[tonic::async_trait]
impl GitHubService for GitHubServiceImpl {
    async fn create_pull_request(
        &self,
        request: Request<PullRequest>,
    ) -> Result<Response<PullRequestDetails>, Status> {
        let pull_request = request.into_inner();

        //create pr request instance
        let pr = LibPullRequest::new(
            &pull_request.owner,
            &pull_request.repo,
            Some(&pull_request.title),
            Some(&pull_request.body),
            &pull_request.base,
            &pull_request.head,
        );

        // Call the create method to create the pull request on GitHub
        match pr.create().await {
            Ok(data) => {
                // If the pull request was created successfully, extract the details
                let pr_details = extract_pr_details(&data);

                // Get the pull request reviews
                //let reviews = get_pull_request_reviews(&pr.owner, &pr.repo, pr.pull_number.unwrap().into()).await.unwrap_or_default();

                // Create the PullRequestDetails instance
                let details = PullRequestDetails {
                    base_ref: pr_details.base_ref,
                    head_ref: pr_details.head_ref,
                    title: pr_details.title,
                    body: pr_details.body,
                    base_sha: pr_details.base_sha,
                    head_sha: pr_details.head_sha,
                };
                Ok(Response::new(details))
            }
            Err(e) => {
                // If there was an error, return it
                Err(Status::internal(format!(
                    "Failed to create pull request: {}",
                    e
                )))
            }
        }
    }

    // Implementation of the fork_repository method
    async fn fork_repository(
        &self,
        request: Request<ForkRequest>,
    ) -> Result<Response<ForkResult>, Status> {
        let fork_request = request.into_inner();

        // Assuming ForkRequest has fields `owner` and `repo`, you can create a new instance like this:
        let fork_request = LibForkRequest {
            owner: &fork_request.owner.clone(),
            repo: &fork_request.repo.clone(),
        };

        // Call the fork method to fork the repository on GitHub
        match fork_request.fork().await {
            Ok(data) => {
                // If the fork was successful, create a ForkResult instance
                let fork_result = ForkResult {
                    owner: data.owner,
                    repo: data.repo,
                    forked_repo: data.forked_repo,
                };
                Ok(Response::new(fork_result))
            }
            Err(e) => {
                // If there was an error, return it
                Err(Status::internal(format!(
                    "Failed to fork repository: {}",
                    e
                )))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let github_service = GitHubServiceImpl::default();

    println!("Server running on {}", addr);

    Server::builder()
        .add_service(GitHubServiceServer::new(github_service))
        .serve(addr)
        .await?;

    Ok(())
}
