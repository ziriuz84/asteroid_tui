pipeline {
    agent { label 'master' }

    environment {
        CARGO_TERM_COLOR = 'always'
        RUST_BACKTRACE = '1'
        GITHUB_REPO = 'ziriuz84/asteroid_tui'
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
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

        stage('Validate Publish') {
            when { buildingTag() }
            steps {
                sh 'cargo publish --dry-run --locked'
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

        stage('SonarQube Analysis') {
            steps {
                script {
                    def sonarVersion = env.TAG_NAME ? env.TAG_NAME.replaceFirst('^v', '') : env.BUILD_NUMBER
                    withSonarQubeEnv('SonarQube-CasaPominiNegri') {
                        sh "sonar-scanner -Dsonar.projectVersion=${sonarVersion}"
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

        stage('Package Release') {
            when { buildingTag() }
            steps {
                sh '''
                    VERSION="${TAG_NAME#v}"
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
            when { buildingTag() }
            steps {
                withCredentials([string(credentialsId: 'github-credentials', variable: 'GH_TOKEN')]) {
                    sh '''
                        gh release create "${TAG_NAME}" \
                            --repo "${GITHUB_REPO}" \
                            --title "asteroid-tui ${TAG_NAME}" \
                            --generate-notes \
                            dist/asteroid-tui-*.tar.gz \
                            dist/asteroid-tui-*.tar.gz.sha256
                    '''
                }
            }
        }

        stage('Publish crates.io') {
            when { buildingTag() }
            steps {
                withCredentials([string(credentialsId: 'crates-io-token', variable: 'CARGO_REGISTRY_TOKEN')]) {
                    sh 'cargo publish --locked --verbose'
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
