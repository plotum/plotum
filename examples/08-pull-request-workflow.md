# Pull request workflow

Every change reaches `main` through a pull request; direct pushes are rejected.

Shows: three section types, cross-section links, dotted "blocked" link.

```plotum
# plotum 1
title: Pull request workflow

section info "Branches"
  main [main | protected]
  feature [feature/login]

section ok "Review"
  pr [Pull request]
  ci [CI | fmt, clippy, test]
  main -> feature : git switch -c
  feature -> pr : push and open PR
  pr -> ci : run checks
  ci --> pr {green} : all green
  pr -> main : merge

section bad "Blocked"
  direct [git push origin main]
  direct ..> main {red} : rejected by the ruleset
```
