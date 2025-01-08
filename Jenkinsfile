pipeline {
    agent {
        label 'agent1' // Specify the label of the agent to use
    }
    environment {
    HOME = "${HOME}"
    CARGO_VERSION = "1.83.0"
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
                                        if ! [ -x "$(command -v rustc)" ]; then
                                            echo "Installing Rustup and Rust..."
                                            wget https://codeload.github.com/rust-lang/rust/tar.gz/refs/tags/${CARGO_VERSION}
                                            ls
                                            ls ${CARGO_VERSION}
                                            export PATH="${HOME}/${CARGO_VERSION}/bin:$PATH"
                                        else
                                            echo "Rust is already installed"
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

