import { Probot } from "probot";

export = (robot: Probot) => {
  robot.on(["pull_request.opened", "pull_request.edited", "pull_request.reopened"], async (context) => {
    /**  Get information about the pull request **/
    const { owner, repo, pull_number: number } = context.pullRequest();
    const pull_number = number ?? context.payload.number;

    const { data } = await context.octokit.pulls.listReviewComments({ owner, repo, pull_number });

    try {
      if (!data || data.length === 0) {
        return;
      }

      const countAuthors = (list: typeof data, line: number | undefined) => {
        if (list.length === 1) {
          return 1;
        } else {
          const authors = list.filter((item) => item.line === line);
          return authors.length;
        }
      };

      await Promise.allSettled(
        data.map(async (comment) => {
          const authors = countAuthors(data, comment.line);
          const bodyText = comment.body.length;
          const body = `${authors === 1 ? "an author" : `${authors} authors`} commented here with ${bodyText} [link to comment](${
            comment.html_url
          }) at ${new Date(comment.created_at)}.`;

          await context.octokit.pulls.createReviewComment({
            ...comment,
            owner,
            repo,
            pull_number,
            body: body,
            commit_id: comment.commit_id,
            line: comment.line,
            side: comment.side,
            start_line: comment.start_line!,
            start_side: comment.start_side ?? undefined,
          });
        })
      );
    } catch (error) {
      robot.log("there seems to be an issue processing this data");
      throw error;
    }
  });
};
