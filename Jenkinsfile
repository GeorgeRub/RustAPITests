pipeline {
    agent {
        label 'agent1' // Specify the label of the agent to use
    }
    environment {
        CARGO_HOME = "${WORKSPACE}/.cargo" // Set up Cargo home
        RUSTUP_HOME = "${WORKSPACE}/.rustup" // Set up Rustup home
        PATH = "${CARGO_HOME}/bin:${PATH}" // Add Cargo binaries to PATH
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
                        export PATH="${WORKSPACE}/.cargo/bin:$PATH"
                    else
                        echo "Rustup already installed"
                    fi
                    rustc --version
                    cargo --version
                    '''
                }
            }
        }
        stage('Checkout Code') {
            steps {
                checkout scm // Check out the code from the configured SCM (e.g., Git)
            }
        }
//         stage('showing of folder'){
//             steps{
//                 sh 'ls $HOME/.cargo/bin'
//             }
//         }
        stage('Build') {
            steps {
            script{
                sh 'cargo build --release' // Build the Rust application in release mode
            }

            }
        }
    }
}

