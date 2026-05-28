pipeline {
    agent any

    environment {
        CARGO_TERM_COLOR = 'always'
        RUST_BACKTRACE = '1'
        GITHUB_REPO = 'ziriuz84/asteroid_tui'
        SONAR_HOST_URL = 'https://sq.casapomininegri.it'
        SONAR_SCANNER_VERSION = '6.2.1.4610'
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
                sh 'git fetch origin --tags --force'
            }
        }

        stage('Detect Release Tag') {
            steps {
                script {
                    def tag = sh(
                        script: "git tag --points-at HEAD --list 'v[0-9]*.[0-9]*.[0-9]*' | sort -V | tail -n 1",
                        returnStdout: true
                    ).trim()
                    if (tag) {
                        env.RELEASE_TAG = tag
                        echo "Release tag on HEAD: ${tag}"
                    } else {
                        env.RELEASE_TAG = ''
                        echo 'No release tag on HEAD; skipping publish and GitHub release stages'
                    }
                }
            }
        }

        stage('Setup Rust') {
            steps {
                sh '''
                    rustup default stable
                    rustup component add clippy llvm-tools-preview
                    cargo install cargo-tarpaulin --locked
                '''
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test --verbose --all-features'
            }
        }

        stage('Coverage') {
            steps {
                catchError(buildResult: 'SUCCESS', stageResult: 'UNSTABLE') {
                    sh 'cargo tarpaulin --verbose --all-features --out Lcov --output-dir .'
                }
            }
        }

        stage('Build Linux') {
            steps {
                sh 'cargo build --release --verbose'
            }
        }


        // Informational only: SonarQube never fails the build (findings or scanner errors).
        stage('SonarQube Analysis') {
            steps {
                catchError(buildResult: 'SUCCESS', stageResult: 'UNSTABLE', message: 'SonarQube analysis failed (non-blocking)') {
                    withCredentials([
                        string(credentialsId: 'SONAR_TOKEN', variable: 'SONAR_TOKEN')
                    ]) {
                        sh '''#!/usr/bin/env bash
                            set -euo pipefail

                            ensure_sonar_scanner() {
                                if command -v sonar-scanner >/dev/null 2>&1; then
                                    return 0
                                fi
                                local scanner_zip="/tmp/sonar-scanner-cli-${SONAR_SCANNER_VERSION}-linux-x64.zip"
                                local scanner_dir="/tmp/sonar-scanner-${SONAR_SCANNER_VERSION}-linux-x64"
                                echo "Downloading SonarScanner ${SONAR_SCANNER_VERSION}..."
                                curl -fsSL \
                                    "https://binaries.sonarsource.com/Distribution/sonar-scanner-cli/sonar-scanner-cli-${SONAR_SCANNER_VERSION}-linux-x64.zip" \
                                    -o "${scanner_zip}"
                                rm -rf "${scanner_dir}"
                                unzip -q "${scanner_zip}" -d /tmp
                                export PATH="${scanner_dir}/bin:${PATH}"
                            }

                            ensure_sonar_scanner

                            sonar-scanner \
                                -Dsonar.host.url="${SONAR_HOST_URL}" \
                                -Dsonar.token="${SONAR_TOKEN}" \
                                -Dsonar.projectVersion="${GIT_COMMIT:-unknown}" \
                                -Dsonar.qualitygate.wait=false
                        '''
                    }
                }
            }
        }
        // stage('Quality Gate') {
        //     steps {
        //         timeout(time: 5, unit: 'MINUTES') {
        //             waitForQualityGate abortPipeline: true
        //         }
        //     }
        // }

        stage('Validate Publish') {
            when {
                expression { env.RELEASE_TAG ==~ /^v\d+\.\d+\.\d+$/ }
            }
            steps {
                sh '''#!/usr/bin/env bash
                    set -euo pipefail
                    CARGO_VER=$(grep -E '^version' Cargo.toml | head -1 | grep -oE '[0-9]+\\.[0-9]+\\.[0-9]+')
                    TAG_VER="${RELEASE_TAG#v}"
                    if [ "$CARGO_VER" != "$TAG_VER" ]; then
                        echo "Cargo.toml version ($CARGO_VER) does not match tag ($TAG_VER)"
                        exit 1
                    fi
                    PKG=$(grep -E '^name[[:space:]]*=' Cargo.toml | head -1 | sed -E 's/^name[[:space:]]*=[[:space:]]*"([^"]+)".*/\\1/')
                    cargo update -p "${PKG}" --precise "${CARGO_VER}"
                    cargo publish --dry-run --locked --allow-dirty
                '''
            }
        }

        stage('Package Release') {
            when {
                expression { env.RELEASE_TAG ==~ /^v\d+\.\d+\.\d+$/ }
            }
            steps {
                sh '''
                    VERSION="${RELEASE_TAG#v}"
                    ARCH="x86_64-unknown-linux-gnu"
                    PACKAGE_DIR="dist/asteroid-tui-${VERSION}-${ARCH}"

                    mkdir -p "${PACKAGE_DIR}"
                    cp target/release/asteroid-tui "${PACKAGE_DIR}/"
                    cp README.md LICENSE "${PACKAGE_DIR}/"

                    tar -czf "dist/asteroid-tui-${VERSION}-${ARCH}.tar.gz" -C dist "asteroid-tui-${VERSION}-${ARCH}"
                    sha256sum "dist/asteroid-tui-${VERSION}-${ARCH}.tar.gz" > "dist/asteroid-tui-${VERSION}-${ARCH}.tar.gz.sha256"
                '''
                archiveArtifacts artifacts: 'dist/*', fingerprint: true
            }
        }

        stage('GitHub Release') {
            when {
                expression { env.RELEASE_TAG ==~ /^v\d+\.\d+\.\d+$/ }
            }
            steps {
                withCredentials([string(credentialsId: 'github-credentials', variable: 'GH_TOKEN')]) {
                    sh '''
                        gh release create "${RELEASE_TAG}" \
                            --repo "${GITHUB_REPO}" \
                            --title "asteroid-tui ${RELEASE_TAG}" \
                            --generate-notes \
                            dist/asteroid-tui-*.tar.gz \
                            dist/asteroid-tui-*.tar.gz.sha256
                    '''
                }
            }
        }

        stage('Publish crates.io') {
            when {
                expression { env.RELEASE_TAG ==~ /^v\d+\.\d+\.\d+$/ }
            }
            steps {
                withCredentials([string(credentialsId: 'crates-io-token', variable: 'CARGO_REGISTRY_TOKEN')]) {
                    sh '''#!/usr/bin/env bash
                        set -euo pipefail
                        CARGO_VER=$(grep -E '^version' Cargo.toml | head -1 | grep -oE '[0-9]+\\.[0-9]+\\.[0-9]+')
                        PKG=$(grep -E '^name[[:space:]]*=' Cargo.toml | head -1 | sed -E 's/^name[[:space:]]*=[[:space:]]*"([^"]+)".*/\\1/')
                        cargo update -p "${PKG}" --precise "${CARGO_VER}"
                        cargo publish --locked --verbose --allow-dirty
                    '''
                }
            }
        }
    }

    post {
        success {
            archiveArtifacts artifacts: 'target/release/asteroid-tui', fingerprint: true, allowEmptyArchive: true
        }
        always {
            cleanWs()
        }
    }
}
