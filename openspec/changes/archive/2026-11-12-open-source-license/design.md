## Context

The kit's tech matrix (agent-kit 0.137.0, § Open-source license) sets `MIT OR Apache-2.0` with both full texts at the root for every public repository. MIT and Apache-2.0 both ask that copies of the software carry the license, and the disk image is a copy.

## Decisions

| Question | Chosen | Why | Rejected |
| --- | --- | --- | --- |
| License | `MIT OR Apache-2.0` | The owner's choice and the kit's rule | MIT alone: no patent grant |
| Apache text | The official `https://www.apache.org/licenses/LICENSE-2.0.txt`, SHA-256 `cfc7749b…3d30` | Unchanged official text | A copy from a crate: some are trimmed |
| Licenses in the release | Copied into `Contents/Resources` of the bundle | Signed with the app, travel with it in the dmg | Loose files in the dmg window: clutter beside the drag target |
| Version | 0.1.1 | A new release that carries the licenses | No release: the owner asked for one |

## Test levels

| Scenario | Level |
|---|---|
| The bundle holds both license texts | E2E `@bundle` scenario on the built bundle |
