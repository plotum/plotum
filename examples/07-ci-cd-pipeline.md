# CI/CD pipeline

From `git push` to production, laid out as a snake.

Shows: `wrap` (rows alternate direction), comments.

```plotum
# plotum 1
title: CI/CD pipeline
subtitle: From push to production

section info "Pipeline"
  wrap: 4
  push [git push]
  lint [Lint | fmt + clippy]
  test [Tests]
  build [Build | Docker image]
  scan [Security scan]
  staging [Staging]
  smoke [Smoke tests]
  prod [Production] {green}

  // first row, left to right
  push -> lint
  lint -> test
  test -> build
  // second row, right to left
  build -> scan
  scan -> staging
  staging -> smoke
  smoke -> prod : manual approval
```
