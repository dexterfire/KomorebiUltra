# Issue tracker: GitHub Issues

Issues and specs live in GitHub Issues of the fork **`dexterfire/KomorebiUltra`** (upstream: `kiskaserver/interactive_assistent`). Use the `gh` CLI.

`gh repo set-default` is pinned to the fork, but always pass `-R dexterfire/KomorebiUltra` explicitly — never file issues or PRs against `upstream` unless the user asks.

## Conventions

- One feature = one **spec issue** (title prefixed `Spec:`), label `enhancement`.
- Implementation tickets are separate issues, one per ticket, linked to the spec as **sub-issues** (`gh api -X POST repos/dexterfire/KomorebiUltra/issues/<spec>/sub_issues -f sub_issue_id=<ticket node id>`), and reference the spec in the body (`Spec: #N`).
- Blocking edges: a `Blocked by: #N, #M` line at the top of the ticket body.
- Triage state = labels (see `triage-labels.md`).
- Discussion goes in issue comments (`gh issue comment`).

## When a skill says "publish to the issue tracker"

`gh issue create -R dexterfire/KomorebiUltra --title "..." --body-file <file> --label <label>`

## When a skill says "fetch the relevant ticket"

`gh issue view <N> -R dexterfire/KomorebiUltra --comments`

## Wayfinding operations

Used by `/wayfinder`.

- **Map**: an issue titled `Map: <effort>` holding Notes / Decisions-so-far / Fog in its body.
- **Child ticket**: an issue, sub-issue of the map, with the question in the body and a `Type:` line (`research`/`prototype`/`grilling`/`task`).
- **Blocking**: `Blocked by: #N, #M` line. Unblocked when every listed issue is closed.
- **Frontier**: open sub-issues of the map that are unblocked and have no assignee; lowest number wins.
- **Claim**: `gh issue edit <N> --add-assignee @me` before any work.
- **Resolve**: comment the answer under `## Answer`, close the issue, then edit the map body's Decisions-so-far with a gist + link.

## Legacy

Earlier tickets live as markdown under `.scratch/<feature>/`; read them if referenced, but file new work on GitHub.
