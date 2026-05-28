## [1.0.1](https://github.com/ziriuz84/asteroid_tui/releases/tag/v1.0.1) (2026-05-28)

### Code Refactoring

- ci: make SonarQube analysis non-blocking ([1facdc0](https://github.com/ziriuz84/asteroid_tui/commit/1facdc0f1ec7705708f78a20e4e34544fc723280))
- ci: run pipeline on any available agent ([d1c81ab](https://github.com/ziriuz84/asteroid_tui/commit/d1c81abc289377e0fe9ba79a8ea639db20aba66c))
- ci: align Jenkins pipeline with server setup ([9d232de](https://github.com/ziriuz84/asteroid_tui/commit/9d232de7d7b78026747e6b63a36045aa71fbfd55))
- ci: add Jenkins pipeline and SonarQube configuration ([8ae6b3e](https://github.com/ziriuz84/asteroid_tui/commit/8ae6b3e82837593ee7beb27e691c81b7e39a8076))

### Chores

- chore(release): add release automation script ([e447c0c](https://github.com/ziriuz84/asteroid_tui/commit/e447c0c96a220bb0221921718531b6e650c7ff57))
---


# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0](https://github.com/ziriuz84/asteroid_tui/releases/tag/v1.0.0) (2026-02-14)

### Features

- **ux:** improve user experience and feedback ([161c6e6](https://github.com/ziriuz84/asteroid_tui/commit/161c6e6ab46afc05ef00e81d4165789679628a01))
- **i18n:** add basic internationalization support ([23aec43](https://github.com/ziriuz84/asteroid_tui/commit/23aec432c083e0e2bc83c4888f3a747417fff628))
- **utils:** add visibility calculation for celestial objects ([bccfed9](https://github.com/ziriuz84/asteroid_tui/commit/bccfed979b9eda68b1362ecf47c385cec35f57c5))
- **settings:** add test mode for default observatory settings ([d34650a](https://github.com/ziriuz84/asteroid_tui/commit/d34650aab4c79fecda975ea5b3105bfa62584c2b))

### Bug Fixes

- **observing_target_list:** make test more flexible ([a460104](https://github.com/ziriuz84/asteroid_tui/commit/a46010471777c5a62b12d6c9100e59f6f5b96a42))
- **utils:** fix deprecated chrono API in tests ([4c90e37](https://github.com/ziriuz84/asteroid_tui/commit/4c90e372f764e020188ebe6d85cf4bbb42294746))
- **utils:** Remove unnecessary log printing ([413e52b](https://github.com/ziriuz84/asteroid_tui/commit/413e52b4830ecc0d53b3d1e8678fe5be370f52b2))

### Code Refactoring

- **tui:** extract common menu validation logic ([cf48225](https://github.com/ziriuz84/asteroid_tui/commit/cf482254168c583b8bc900accc9ef403ad216bde))
- **settings:** improve error handling in config management ([25ade14](https://github.com/ziriuz84/asteroid_tui/commit/25ade143fce5bcb0d71073ed82f2975a950461a4))
- **sun_moon_times:** improve error handling and remove unwrap() ([2412ca8](https://github.com/ziriuz84/asteroid_tui/commit/2412ca80eea646a1025ab2c9156a3d22141b0f17))
- **weather:** improve error handling and remove unwrap() ([ded7adf](https://github.com/ziriuz84/asteroid_tui/commit/ded7adf372287cb7783c818138523a5fd3205140))

### Documentation

- add commit messages documentation ([0e83e85](https://github.com/ziriuz84/asteroid_tui/commit/0e83e85b49eadeb01d2f7fe2021611b64722173f))
- **utils:** add module documentation for utilities library ([1c1a94d](https://github.com/ziriuz84/asteroid_tui/commit/1c1a94d484e81f1329d2111d0c8f95d151b80993))

### Chores

- **release:** 1.0.0 ([13ef509](https://github.com/ziriuz84/asteroid_tui/commit/13ef509babfa023450e27bebc8bf1a89df5d88df))
- **release:** 1.0.0 ([1c321b2](https://github.com/ziriuz84/asteroid_tui/commit/1c321b2685510caffa48073394b84f8dadae93d2))
- **dependencies:** update promkit to version 0.8.0 ([9a76083](https://github.com/ziriuz84/asteroid_tui/commit/9a760838b7fb73206c50bb4a21581b3b4390002c))
