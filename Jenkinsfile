pipeline {
    agent {
        label 'agent1' // Specify the label of the agent to use
    }
    environment {
        CARGO_HOME = "${WORKSPACE}/.cargo" // Set up Cargo home
        RUSTUP_HOME = "${WORKSPACE}/.rustup" // Set up Rustup home
    }
    stages {
        stage('Setup') {
            steps {
                script {
                    // Ensure Rust is installed on the agent
                    sh '''
                    if ! [ -x "$(command -v rustup)" ]; then
                        echo "Installing Rustup..."
                        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                        export PATH="$HOME/.cargo/bin:$PATH"
                    else
                        echo "Rustup already installed"
                    fi
                    rustup show
                    '''
                }
            }
        }
        stage('Checkout Code') {
            steps {
                checkout scm // Check out the code from the configured SCM (e.g., Git)
            }
        }
        stage('Build') {
            steps {
                sh 'cargo build --release' // Build the Rust application in release mode
            }
        }
//         stage('Test') {
//             steps {
//                 sh 'cargo test' // Run tests for the application
//             }
//         }
//         stage('Package') {
//             steps {
//                 script {
//                     // Package the built binary
//                     def appName = "my_rust_app"
//                     def outputDir = "${WORKSPACE}/dist"
//                     sh """
//                     mkdir -p ${outputDir}
//                     cp target/release/${appName} ${outputDir}/
//                     echo "Packaged ${appName} into ${outputDir}"
//                     """
//                 }
//             }
//         }
//     }
//     post {
//         always {
//             archiveArtifacts artifacts: 'dist/**', fingerprint: true // Archive the output files
//             cleanWs() // Clean up the workspace
//         }
//     }
}



// pipeline {
//     agent {
//         label 'agent1'
//       }
//     stages {
//
//         stage('Build') {
//           steps {
//           sh "cargo version"
//             sh "cargo build --release"
//           }
//
//         }
//       }
// }