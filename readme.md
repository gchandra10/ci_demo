## Workflow

- Triggers on pushes and pull requests to the main branch.
- Sets up the job on an Ubuntu runner.
- Checks out your repository.
- Installs Rust using the stable toolchain.
- Builds your project in release mode.
- Runs tests.
- Notify via Discord Webhook.

## Ensure Failfast (Default behavior)

GitHub Actions follows a fail-fast approach by default. This means if any step in a job fails, the subsequent steps are not executed. You don't need to add any special configuration for this behavior.

## Actions

In GitHub Actions workflows, the ```uses``` keyword is used to specify a GitHub Action or a Docker container to use as part of a job. 

### GitHub Actions (uses: actions/...):

These are pre-built actions provided by GitHub or created by the community. They can perform a wide range of tasks like checking out your code, setting up a programming environment, publishing packages, etc. For example:

**actions/checkout@v2**: This is a widely-used action that checks out your repository under $GITHUB_WORKSPACE, so your workflow can access it.

**actions-rs/toolchain@v1**: This action sets up a Rust toolchain for use in the workflow, which is essential for building and testing Rust projects.

### Docker Containers (uses: docker://...):

### Versioning (@v2, @v1, etc.): 

The part after the @ symbol specifies the version of the action or Docker container. It's important to specify this to ensure your workflow uses a stable and tested version of the action. 

*Other ways to call actions*

uses: actions/checkout@main

## Best Practice

- Using a specific version (like @v2) is generally recommended for stability. This ensures that your workflows don't break if the action's authors push a breaking change to the default branch.
- Automatically using the latest version can also pose a security risk, especially if the action is not from a trusted source. It also helps to ensure reproducibility and realibility of your pipelines. Also remember, keeping older version also pose a security risk. So periodically check whats latest :)

