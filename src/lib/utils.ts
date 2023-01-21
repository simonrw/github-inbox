export interface PanelArgs {
  title: string;
  query: string;
}

export const queryMapping = (role: string): PanelArgs => {
  const mapping = {
    "created-issues": {
      title: "Created Issues",
      query: "fetch_created_issues",
    },
    "assigned-issues": {
      title: "Assigned Issues",
      query: "fetch_assigned_issues",
    },
    "mentioned-issues": {
      title: "Mentioned Issues",
      query: "fetch_mentioned_issues",
    },
    "created-prs": {
      title: "Created PRs",
      query: "fetch_created_prs",
    },
    "assigned-prs": {
      title: "Assigned PRs",
      query: "fetch_assigned_prs",
    },
    "mentioned-prs": {
      title: "Mentioned PRs",
      query: "fetch_mentioned_prs",
    },
    "review-requests": {
      title: "Review Requests",
      query: "fetch_review_requests",
    },
  };

  const result = mapping[role];
  if (!result) {
    throw `Could not find panel arguments for role ${role}`;
  }
  return result;
};
