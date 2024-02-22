use tonic::{transport::Server, Request, Response, Status};
use fresheyes::git_hub_service_server::{GitHubService, GitHubServiceServer};
use fresheyes::{PullRequest, PullRequestDetails};

pub mod fresheyes{
    tonic::include_proto!("fresheyes"); // The string specified here must match the proto package name
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

        // Create a PullRequest instance using the new method
        let pr = PullRequest::new(
            &pull_request.owner,
            &pull_request.repo,
            pull_request.title.as_deref(),
            pull_request.body.as_deref(),
            &pull_request.base,
            &pull_request.head,
        );

        // Call the create method to create the pull request on GitHub
        match pr.create().await {
            Ok(data) => {
                // If the pull request was created successfully, extract the details
                let pr_details = extract_pr_details(&data).await;

                // Get the pull request reviews
                let reviews = get_pull_request_reviews(&pr.owner, &pr.repo, pr.pull_number.clone().unwrap().into()).await;

                // Create the PullRequestDetails instance
                let details = PullRequestDetails {
                    base_ref: pr_details.base_ref,
                    head_ref: pr_details.head_ref,
                    title: pr_details.title,
                    body: pr_details.body,
                    base_sha: pr_details.base_sha,
                    head_sha: pr_details.head_sha,
                    reviews: reviews.len() as i32, // assuming you want to return the number of reviews
                };
                Ok(Response::new(details))
            }
            Err(e) => {
                // If there was an error, return it
                Err(Status::internal(format!("Failed to create pull request: {}", e)))
            }
        }
    }

    // further methods will be added here
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let github_service = GitHubServiceImpl::default();

    println!("Server running on {addr}");

    Server::builder()
        .add_service(GitHubServiceServer::new(github_service))
        .serve(addr)
        .await?;

    Ok(())
}
