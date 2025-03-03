# Information on Contributing

Contributions are absolutely welcome, but there are a few things to note. This 
document mostly exists just in case someone decides to start contributing so 
they can be aware of some important stuff. None of the statements below are 
explicit rules and are probably subject to change or being completely ignored at
some point in the future. 

Well... except the one about GitLab versus GitHub.

## Feature requests / suggestions / ideas
This is not a paid product. It's also currently maintained/developed by a single
person as a hobby project. Feature requests / feature suggestions / changes are
entirely down to whether someone wants to implement them or not, and how those 
changes affect the rest of the codebase.

Simpler codebases are easier and more enjoyable to maintain. So having fewer, 
**but most useful**, features would probably be a good thing to 
contemplate when making requests/suggestions/sharing ideas.

## Development work on GitLab, not GitHub

All development work happens in the main GitLab project 
(https://gitlab.com/octatools/octatools).

Creating issues in GitHub is fine, and they'll be treated as "user reported 
issues". GitLab issues will likely lean more towards "developer work planning" 
tickets / issues (will likely contain notes/extra detail/some excuses about why 
I haven't looked at a thing yet).

Any PRs in GitHub will be closed. 
The steps for moving a PR over to GitLab are pretty simple
1. Create a GitLab account: https://gitlab.com/users/sign_up
2. Set up an SSH key for GitLab:
   https://docs.gitlab.co.jp/ee/user/ssh.html#add-an-ssh-key-to-your-gitlab-account
3. Fork the project: https://gitlab.com/octatools/octatools/-/forks/new
4. Change your local origin remote's url:
   `git remote origin set-url ssh://YOUR_NEW_FORK_SSH_ADDRESS`

## GitLab open source program
This project is part of the GitLab Open Source Projects Program 
(https://about.gitlab.com/solutions/open-source/), which means that all CI 
pipelines run at discounted rates.

Merge request CI pipelines currently run at a 0.8% discount compared to normal
pipelines, meaning you can effectively get up to 50k compute minutes for any of 
your merge requests in this project (caveats apply based on CI instance types).

## Coding conventions / style guide
There are linting checks in merge request pipelines that should ideally be 
adhered to. But working software is the higher priority.

## Contributor conduct
Don't be a jerk. Or at least too much of a jerk. Apologise if you have been a 
jerk.

## License applied to contributions
The entire project is licensed under GPL-v3.0.
All contributions added to the project will be licensed as GPL-v3.0.
Please ensure code contributions are GPL-v3.0 compatible.

## Reverse engineering the OctaTrack OS
Reverse engineering the machine's operating system is absolutely not okay.
Please do not suggest it or attempt it.

## Other hardware
Other machines like the Elektron Digikat won't be supported as part of this 
project. Please don't contribute code for other machines, or ask for this 
project to be replicated for another machine (i've already spent way too much of
my personal free time on this).

## Merge process (and cases where merged changed are reverted)
Merge requests are merged by performing a "squash merge" -- all MR commits are 
squashed into a single commit and that commit is added to the `main` branch. 

The only people capable of merging changes are Maintainers, which is currently 
@dijksterhuis

Merge request pipelines currently only test code changes against the 
`x86_64-uknown-linux-gnu` target (this may change in the future to include other 
targets). Once changes have been merged into `main`, the changes are then tested
for other build targets. If there are issues with builds **the 
merged changes will be reverted**.

You will need to create a new merge request, where the first commit is reverting
the revert of your original contribution.

I dislike hotfixes being made on `main`, or multiple hotfix merge requests being
made as new issues are found. It's just simpler and cleaner to revert.

Might not apply in all cases, but it's important to know that I might revert 
your changes if things are borked.

Note -- i am going to look at adding other build targets to merge request 
pipelines to try and mitigate the whole not knowing the build will fail ahead of
time problem.

## Release process
Releases are built from git tags pushed to GitHub.
This GitLab repository syncs changes to all protected branches/tags to the 
GitHub release repository, where a new release is created when a tag is pushed.

The only people capable of pushing new tags are Maintainers, which is currently
@dijksterhuis