# Hacking Outscale SDK

SDK itself is generated from Outscale's [OpenAPI description](https://github.com/outscale/osc-api).
Any change directly applied to source code will be overwritten at generation time.

Examples and tests are welcome!

# Versioning

This SDK follows [semantic versioning](https://semver.org/) from the SDK perspective (not API).
Some events may trigger a major (breaking) version of the SDK:
1. OpenAPI generator introduce a new major version.
2. Outscale introduce a new major version of its API.

When OpenAPI generator introduce a breaking change, SDK can be generated in several versions (see corresponding branches).

# Generate SDK

1. Have some tools ready: GNU make, git, docker.
2. Edit `api_version` file and to the latest Outscale API version.
3. Edit `sdk_version` file and change it according to [semantic versioning](https://semver.org/).
4. Launch sdk generation by running `make gen`.

Under the hood it get official Outscale yaml and run openapi-generator through docker.

# Sending a Merge Request

If you plan to make some change in source code, consider making a pull request in [openapi-generator project](https://github.com/OpenAPITools/openapi-generator/).

Otherwise:
- Your merge request must be rebased on the latest commit.
- Be sure that tests still pass by running `make test`.

# How to release

1. Be sure have the latest version from repository.
2. Update `api_version` to the last Outscale API version.
3. Update `sdk_version` following [semantic versioning](https://semver.org/) logic.
4. `make gen` to re-build the sdk.
5. `make test` and fix any issue.
6. Commit changes.
7. Create PR.
8. Review and merge PR.
9. Create and push new sdk version tag.
10. Create new release from tag.
11. Publish crate following [crates.io guidelines](https://doc.rust-lang.org/cargo/reference/publishing.html).
