import { Probot } from "probot";

export = (robot: Probot) => {
  robot.on(["pull_request.opened"], async (context) => {
    /**  Get information about the pull request **/
    const { owner, repo, pull_number: number } = context.pullRequest();
    const pull_number = number ?? context.payload.number;

    const { data } = await context.octokit.pulls.listReviewComments({ owner, repo, pull_number });

    try {
      if (!data || data.length === 0) {
        return;
      }

      const groupComments: Record<string, Array<typeof data>> = data
        .map((x: any) => ({ ...x, line: String(x.line) }))
        .reduce((acc, curr) => {
          const key = curr.line;

          const group = acc[key] ?? [];

          return { ...acc, [key]: [...group, curr] };
        }, {});

      await Promise.all(
        Object.entries(groupComments).map(async ([k, v]) => {
          const list = v.flat().map((x) => ({ html_url: x.html_url, created_at: x.created_at }));

          const formatString = list
            .map((val, idx) => {
              return `- [comment link ${idx + 1}](${val.html_url}) at ${new Date(val.created_at).toLocaleString()}`;
            })
            .join("\n");

          const body = `${v.length === 1 ? "An author" : `${v.length} authors`} commented here with\n\n${formatString}.`;

          const comment = v.flat()[0];
          const res = await context.octokit.pulls.createReviewComment({
            owner,
            repo,
            pull_number,
            body: body,
            commit_id: comment.commit_id,
            path: comment.path,
            side: comment.side,
            line: Number(k),
          });

          return res;
        })
      );
    } catch (error) {
      robot.log("there seems to be an issue processing this data");
      throw error;
    }
  });
};
