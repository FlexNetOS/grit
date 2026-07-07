# Changelog

## [0.6.4](https://github.com/FlexNetOS/grit/compare/grit-v0.6.3...grit-v0.6.4) (2026-07-07)


### Features

* add --wait flag to claim command for retry with backoff ([13ab70b](https://github.com/FlexNetOS/grit/commit/13ab70bd71887f8601f39f3d203f65c59644bf4d)), closes [#11](https://github.com/FlexNetOS/grit/issues/11)
* add C#, Go, Java, C, C++, Ruby language support ([b7c68e8](https://github.com/FlexNetOS/grit/commit/b7c68e8bb9aa0b5161e7683ccbc3cfa4ed42d508))
* add C#, Go, Java, C, C++, Ruby language support ([08fef9b](https://github.com/FlexNetOS/grit/commit/08fef9b250a03c398a7dbe19748043b2dede7519))
* add PHP language support (11 languages total) ([94ec393](https://github.com/FlexNetOS/grit/commit/94ec393218b457a99393797611457e2e6562466f))
* add PHP language support (11 languages total) ([a1ccaf1](https://github.com/FlexNetOS/grit/commit/a1ccaf143a36a8a8f6e64aba8dd0a5dfc0de7b16))
* add Swift and Kotlin language support with kind detection ([bc9f1fa](https://github.com/FlexNetOS/grit/commit/bc9f1fa0956949649b2f64c71316ba17cbe1522a)), closes [#14](https://github.com/FlexNetOS/grit/issues/14)
* add symbol reconciliation command ([9698e59](https://github.com/FlexNetOS/grit/commit/9698e59a674a70c19d4d6035eccd49efa94e096c))
* add union-step-2 symbol reconciliation ([c23c072](https://github.com/FlexNetOS/grit/commit/c23c072071bac5bf5cfd279599af1bc1ced16d53))
* assign, queue, S3 events, dependency-aware locking ([#12](https://github.com/FlexNetOS/grit/issues/12) [#13](https://github.com/FlexNetOS/grit/issues/13) [#15](https://github.com/FlexNetOS/grit/issues/15) [#17](https://github.com/FlexNetOS/grit/issues/17)) ([#19](https://github.com/FlexNetOS/grit/issues/19)) ([a2c4873](https://github.com/FlexNetOS/grit/commit/a2c48735e0a16c49ca1541c4865fce438c479405))
* real shared read locks on S3/Azure backends ([#16](https://github.com/FlexNetOS/grit/issues/16)) ([#25](https://github.com/FlexNetOS/grit/issues/25)) ([59f349b](https://github.com/FlexNetOS/grit/commit/59f349be7ecf62e028587b74018ac7ebe4f18ae0))


### Bug Fixes

* **ci:** draft release before asset upload ([4ab1890](https://github.com/FlexNetOS/grit/commit/4ab18904a95984d4db163e99731e7d8f28a42301))
* **ci:** load release-please manifest config ([fceaeb6](https://github.com/FlexNetOS/grit/commit/fceaeb64cce62fb81dae14f1ec081e55ad2a7826))
* **ci:** skip upstream homebrew publish in forks ([7ab5112](https://github.com/FlexNetOS/grit/commit/7ab5112cfdb9701bb6c405ac7e6ac26cd53997f5))
* close lock-protocol P0/P1 holes (atomic claim, merge-lock liveness, session branch) ([#23](https://github.com/FlexNetOS/grit/issues/23)) ([8ad1fa5](https://github.com/FlexNetOS/grit/commit/8ad1fa5e654f826e96a266e45fb73a6f7f1a0e21))
* make S3 lock acquisition fail closed, never silently non-atomic (P0) ([#24](https://github.com/FlexNetOS/grit/issues/24)) ([ae8bded](https://github.com/FlexNetOS/grit/commit/ae8bded9a0760f8cbc06b2fa013fb320f1d16b2b))
* P2/P3 cloud-backend hardening (refresh resurrection, Event Grid doc) ([#28](https://github.com/FlexNetOS/grit/issues/28)) ([c75aea9](https://github.com/FlexNetOS/grit/commit/c75aea96c3a32527e3b6c2b672030f98fc4da687))
* P2/P3 hardening — queue FIFO, availability, migrations, git robustness ([#27](https://github.com/FlexNetOS/grit/issues/27)) ([db8b068](https://github.com/FlexNetOS/grit/commit/db8b0685243f3fac9de314a2f457296edd799a39))
* prevent repo corruption and commit loss in `grit done` ([#21](https://github.com/FlexNetOS/grit/issues/21)) ([#22](https://github.com/FlexNetOS/grit/issues/22)) ([65f0ebb](https://github.com/FlexNetOS/grit/commit/65f0ebbd3a3dbcaf1eeaabf751a57b187c19b97e))
* resilient parser — skip incompatible grammars instead of aborting ([552e5b7](https://github.com/FlexNetOS/grit/commit/552e5b79384228fc1b53e7dae851a84e6e3e5c90))
* security hardening, dead code removal, dependency cleanup ([e8b68a8](https://github.com/FlexNetOS/grit/commit/e8b68a896f6e0774ceb19ff9039852b864def516))
* skip incompatible language grammars instead of aborting scan ([6dc40de](https://github.com/FlexNetOS/grit/commit/6dc40de782d912d860011dd278823d0d396114f9))

## [0.6.3](https://github.com/FlexNetOS/grit/compare/grit-v0.6.2...grit-v0.6.3) (2026-07-07)


### Bug Fixes

* **ci:** skip upstream homebrew publish in forks ([7ab5112](https://github.com/FlexNetOS/grit/commit/7ab5112cfdb9701bb6c405ac7e6ac26cd53997f5))

## [0.6.2](https://github.com/FlexNetOS/grit/compare/grit-v0.6.1...grit-v0.6.2) (2026-07-07)


### Features

* add --wait flag to claim command for retry with backoff ([13ab70b](https://github.com/FlexNetOS/grit/commit/13ab70bd71887f8601f39f3d203f65c59644bf4d)), closes [#11](https://github.com/FlexNetOS/grit/issues/11)
* add C#, Go, Java, C, C++, Ruby language support ([b7c68e8](https://github.com/FlexNetOS/grit/commit/b7c68e8bb9aa0b5161e7683ccbc3cfa4ed42d508))
* add C#, Go, Java, C, C++, Ruby language support ([08fef9b](https://github.com/FlexNetOS/grit/commit/08fef9b250a03c398a7dbe19748043b2dede7519))
* add PHP language support (11 languages total) ([94ec393](https://github.com/FlexNetOS/grit/commit/94ec393218b457a99393797611457e2e6562466f))
* add PHP language support (11 languages total) ([a1ccaf1](https://github.com/FlexNetOS/grit/commit/a1ccaf143a36a8a8f6e64aba8dd0a5dfc0de7b16))
* add Swift and Kotlin language support with kind detection ([bc9f1fa](https://github.com/FlexNetOS/grit/commit/bc9f1fa0956949649b2f64c71316ba17cbe1522a)), closes [#14](https://github.com/FlexNetOS/grit/issues/14)
* add symbol reconciliation command ([9698e59](https://github.com/FlexNetOS/grit/commit/9698e59a674a70c19d4d6035eccd49efa94e096c))
* add union-step-2 symbol reconciliation ([c23c072](https://github.com/FlexNetOS/grit/commit/c23c072071bac5bf5cfd279599af1bc1ced16d53))
* assign, queue, S3 events, dependency-aware locking ([#12](https://github.com/FlexNetOS/grit/issues/12) [#13](https://github.com/FlexNetOS/grit/issues/13) [#15](https://github.com/FlexNetOS/grit/issues/15) [#17](https://github.com/FlexNetOS/grit/issues/17)) ([#19](https://github.com/FlexNetOS/grit/issues/19)) ([a2c4873](https://github.com/FlexNetOS/grit/commit/a2c48735e0a16c49ca1541c4865fce438c479405))
* real shared read locks on S3/Azure backends ([#16](https://github.com/FlexNetOS/grit/issues/16)) ([#25](https://github.com/FlexNetOS/grit/issues/25)) ([59f349b](https://github.com/FlexNetOS/grit/commit/59f349be7ecf62e028587b74018ac7ebe4f18ae0))


### Bug Fixes

* **ci:** draft release before asset upload ([4ab1890](https://github.com/FlexNetOS/grit/commit/4ab18904a95984d4db163e99731e7d8f28a42301))
* **ci:** load release-please manifest config ([fceaeb6](https://github.com/FlexNetOS/grit/commit/fceaeb64cce62fb81dae14f1ec081e55ad2a7826))
* close lock-protocol P0/P1 holes (atomic claim, merge-lock liveness, session branch) ([#23](https://github.com/FlexNetOS/grit/issues/23)) ([8ad1fa5](https://github.com/FlexNetOS/grit/commit/8ad1fa5e654f826e96a266e45fb73a6f7f1a0e21))
* make S3 lock acquisition fail closed, never silently non-atomic (P0) ([#24](https://github.com/FlexNetOS/grit/issues/24)) ([ae8bded](https://github.com/FlexNetOS/grit/commit/ae8bded9a0760f8cbc06b2fa013fb320f1d16b2b))
* P2/P3 cloud-backend hardening (refresh resurrection, Event Grid doc) ([#28](https://github.com/FlexNetOS/grit/issues/28)) ([c75aea9](https://github.com/FlexNetOS/grit/commit/c75aea96c3a32527e3b6c2b672030f98fc4da687))
* P2/P3 hardening — queue FIFO, availability, migrations, git robustness ([#27](https://github.com/FlexNetOS/grit/issues/27)) ([db8b068](https://github.com/FlexNetOS/grit/commit/db8b0685243f3fac9de314a2f457296edd799a39))
* prevent repo corruption and commit loss in `grit done` ([#21](https://github.com/FlexNetOS/grit/issues/21)) ([#22](https://github.com/FlexNetOS/grit/issues/22)) ([65f0ebb](https://github.com/FlexNetOS/grit/commit/65f0ebbd3a3dbcaf1eeaabf751a57b187c19b97e))
* resilient parser — skip incompatible grammars instead of aborting ([552e5b7](https://github.com/FlexNetOS/grit/commit/552e5b79384228fc1b53e7dae851a84e6e3e5c90))
* security hardening, dead code removal, dependency cleanup ([e8b68a8](https://github.com/FlexNetOS/grit/commit/e8b68a896f6e0774ceb19ff9039852b864def516))
* skip incompatible language grammars instead of aborting scan ([6dc40de](https://github.com/FlexNetOS/grit/commit/6dc40de782d912d860011dd278823d0d396114f9))

## [0.6.1](https://github.com/FlexNetOS/grit/compare/v0.6.0...v0.6.1) (2026-07-07)


### Bug Fixes

* **ci:** draft release before asset upload ([4ab1890](https://github.com/FlexNetOS/grit/commit/4ab18904a95984d4db163e99731e7d8f28a42301))

## [0.6.0](https://github.com/FlexNetOS/grit/compare/v0.5.0...v0.6.0) (2026-07-07)


### Features

* add symbol reconciliation command ([9698e59](https://github.com/FlexNetOS/grit/commit/9698e59a674a70c19d4d6035eccd49efa94e096c))
* add union-step-2 symbol reconciliation ([c23c072](https://github.com/FlexNetOS/grit/commit/c23c072071bac5bf5cfd279599af1bc1ced16d53))

## [0.5.0](https://github.com/FlexNetOS/grit/compare/v0.4.0...v0.5.0) (2026-06-27)


### Features

* add --wait flag to claim command for retry with backoff ([13ab70b](https://github.com/FlexNetOS/grit/commit/13ab70bd71887f8601f39f3d203f65c59644bf4d)), closes [#11](https://github.com/FlexNetOS/grit/issues/11)
* add C#, Go, Java, C, C++, Ruby language support ([b7c68e8](https://github.com/FlexNetOS/grit/commit/b7c68e8bb9aa0b5161e7683ccbc3cfa4ed42d508))
* add C#, Go, Java, C, C++, Ruby language support ([08fef9b](https://github.com/FlexNetOS/grit/commit/08fef9b250a03c398a7dbe19748043b2dede7519))
* add PHP language support (11 languages total) ([94ec393](https://github.com/FlexNetOS/grit/commit/94ec393218b457a99393797611457e2e6562466f))
* add PHP language support (11 languages total) ([a1ccaf1](https://github.com/FlexNetOS/grit/commit/a1ccaf143a36a8a8f6e64aba8dd0a5dfc0de7b16))
* add Swift and Kotlin language support with kind detection ([bc9f1fa](https://github.com/FlexNetOS/grit/commit/bc9f1fa0956949649b2f64c71316ba17cbe1522a)), closes [#14](https://github.com/FlexNetOS/grit/issues/14)
* assign, queue, S3 events, dependency-aware locking ([#12](https://github.com/FlexNetOS/grit/issues/12) [#13](https://github.com/FlexNetOS/grit/issues/13) [#15](https://github.com/FlexNetOS/grit/issues/15) [#17](https://github.com/FlexNetOS/grit/issues/17)) ([#19](https://github.com/FlexNetOS/grit/issues/19)) ([a2c4873](https://github.com/FlexNetOS/grit/commit/a2c48735e0a16c49ca1541c4865fce438c479405))
* real shared read locks on S3/Azure backends ([#16](https://github.com/FlexNetOS/grit/issues/16)) ([#25](https://github.com/FlexNetOS/grit/issues/25)) ([59f349b](https://github.com/FlexNetOS/grit/commit/59f349be7ecf62e028587b74018ac7ebe4f18ae0))


### Bug Fixes

* close lock-protocol P0/P1 holes (atomic claim, merge-lock liveness, session branch) ([#23](https://github.com/FlexNetOS/grit/issues/23)) ([8ad1fa5](https://github.com/FlexNetOS/grit/commit/8ad1fa5e654f826e96a266e45fb73a6f7f1a0e21))
* make S3 lock acquisition fail closed, never silently non-atomic (P0) ([#24](https://github.com/FlexNetOS/grit/issues/24)) ([ae8bded](https://github.com/FlexNetOS/grit/commit/ae8bded9a0760f8cbc06b2fa013fb320f1d16b2b))
* P2/P3 cloud-backend hardening (refresh resurrection, Event Grid doc) ([#28](https://github.com/FlexNetOS/grit/issues/28)) ([c75aea9](https://github.com/FlexNetOS/grit/commit/c75aea96c3a32527e3b6c2b672030f98fc4da687))
* P2/P3 hardening — queue FIFO, availability, migrations, git robustness ([#27](https://github.com/FlexNetOS/grit/issues/27)) ([db8b068](https://github.com/FlexNetOS/grit/commit/db8b0685243f3fac9de314a2f457296edd799a39))
* prevent repo corruption and commit loss in `grit done` ([#21](https://github.com/FlexNetOS/grit/issues/21)) ([#22](https://github.com/FlexNetOS/grit/issues/22)) ([65f0ebb](https://github.com/FlexNetOS/grit/commit/65f0ebbd3a3dbcaf1eeaabf751a57b187c19b97e))
* resilient parser — skip incompatible grammars instead of aborting ([552e5b7](https://github.com/FlexNetOS/grit/commit/552e5b79384228fc1b53e7dae851a84e6e3e5c90))
* security hardening, dead code removal, dependency cleanup ([e8b68a8](https://github.com/FlexNetOS/grit/commit/e8b68a896f6e0774ceb19ff9039852b864def516))
* skip incompatible language grammars instead of aborting scan ([6dc40de](https://github.com/FlexNetOS/grit/commit/6dc40de782d912d860011dd278823d0d396114f9))

## [0.4.0](https://github.com/rtk-ai/grit/compare/v0.3.0...v0.4.0) (2026-06-14)


### Features

* real shared read locks on S3/Azure backends ([#16](https://github.com/rtk-ai/grit/issues/16)) ([#25](https://github.com/rtk-ai/grit/issues/25)) ([59f349b](https://github.com/rtk-ai/grit/commit/59f349be7ecf62e028587b74018ac7ebe4f18ae0))


### Bug Fixes

* close lock-protocol P0/P1 holes (atomic claim, merge-lock liveness, session branch) ([#23](https://github.com/rtk-ai/grit/issues/23)) ([8ad1fa5](https://github.com/rtk-ai/grit/commit/8ad1fa5e654f826e96a266e45fb73a6f7f1a0e21))
* make S3 lock acquisition fail closed, never silently non-atomic (P0) ([#24](https://github.com/rtk-ai/grit/issues/24)) ([ae8bded](https://github.com/rtk-ai/grit/commit/ae8bded9a0760f8cbc06b2fa013fb320f1d16b2b))
* P2/P3 cloud-backend hardening (refresh resurrection, Event Grid doc) ([#28](https://github.com/rtk-ai/grit/issues/28)) ([c75aea9](https://github.com/rtk-ai/grit/commit/c75aea96c3a32527e3b6c2b672030f98fc4da687))
* P2/P3 hardening — queue FIFO, availability, migrations, git robustness ([#27](https://github.com/rtk-ai/grit/issues/27)) ([db8b068](https://github.com/rtk-ai/grit/commit/db8b0685243f3fac9de314a2f457296edd799a39))
* prevent repo corruption and commit loss in `grit done` ([#21](https://github.com/rtk-ai/grit/issues/21)) ([#22](https://github.com/rtk-ai/grit/issues/22)) ([65f0ebb](https://github.com/rtk-ai/grit/commit/65f0ebbd3a3dbcaf1eeaabf751a57b187c19b97e))

## [0.3.0](https://github.com/rtk-ai/grit/compare/v0.2.0...v0.3.0) (2026-04-06)


### Features

* assign, queue, S3 events, dependency-aware locking ([#12](https://github.com/rtk-ai/grit/issues/12) [#13](https://github.com/rtk-ai/grit/issues/13) [#15](https://github.com/rtk-ai/grit/issues/15) [#17](https://github.com/rtk-ai/grit/issues/17)) ([#19](https://github.com/rtk-ai/grit/issues/19)) ([a2c4873](https://github.com/rtk-ai/grit/commit/a2c48735e0a16c49ca1541c4865fce438c479405))

## [0.2.0](https://github.com/rtk-ai/grit/compare/v0.1.0...v0.2.0) (2026-03-28)


### Features

* add --wait flag to claim command for retry with backoff ([13ab70b](https://github.com/rtk-ai/grit/commit/13ab70bd71887f8601f39f3d203f65c59644bf4d)), closes [#11](https://github.com/rtk-ai/grit/issues/11)
* add C#, Go, Java, C, C++, Ruby language support ([b7c68e8](https://github.com/rtk-ai/grit/commit/b7c68e8bb9aa0b5161e7683ccbc3cfa4ed42d508))
* add C#, Go, Java, C, C++, Ruby language support ([08fef9b](https://github.com/rtk-ai/grit/commit/08fef9b250a03c398a7dbe19748043b2dede7519))
* add PHP language support (11 languages total) ([94ec393](https://github.com/rtk-ai/grit/commit/94ec393218b457a99393797611457e2e6562466f))
* add PHP language support (11 languages total) ([a1ccaf1](https://github.com/rtk-ai/grit/commit/a1ccaf143a36a8a8f6e64aba8dd0a5dfc0de7b16))
* add Swift and Kotlin language support with kind detection ([bc9f1fa](https://github.com/rtk-ai/grit/commit/bc9f1fa0956949649b2f64c71316ba17cbe1522a)), closes [#14](https://github.com/rtk-ai/grit/issues/14)


### Bug Fixes

* resilient parser — skip incompatible grammars instead of aborting ([552e5b7](https://github.com/rtk-ai/grit/commit/552e5b79384228fc1b53e7dae851a84e6e3e5c90))
* skip incompatible language grammars instead of aborting scan ([6dc40de](https://github.com/rtk-ai/grit/commit/6dc40de782d912d860011dd278823d0d396114f9))

## 0.1.0 (2026-03-17)


### Bug Fixes

* security hardening, dead code removal, dependency cleanup ([e8b68a8](https://github.com/rtk-ai/grit/commit/e8b68a896f6e0774ceb19ff9039852b864def516))
