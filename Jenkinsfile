pipeline {
    agent {
        label 'agent1' // Specify the label of the agent to use
    }
    environment {
        HOME = "${HOME}"
        CARGO_VERSION = "1.83.0"
        RUST_PACAGE = "${CARGO_VERSION}.tar.gz"
        CARGO_HOME = "${WORKSPACE}/.cargo" // Set up Cargo home
        RUSTUP_HOME = "rust-${CARGO_VERSION}/.rustup" // Set up Rustup home
        PATH = "${CARGO_HOME}/bin:${PATH}" // Add Cargo binaries to PATH
    }
    stages {
        stage('Setup') {
            steps {
                script {
                //https://github.com/rust-lang/rust/archive/refs/tags/1.83.0.tar.gz
                    // Ensure Rust is installed on the agent  https://github.com/rust-lang/rust/archive/refs/tags/1.83.0.tar.gz
                    sh '''
                                        if ! [ -x "$(command -v rustc)" ]; then
                                            echo "Installing Rustup and Rust..."
                                            wget https://github.com/rust-lang/rust/archive/refs/tags/${RUST_PACAGE}
                                            tar -xvzf ${RUST_PACAGE}
                                            ls
//                                             ls ./${RUSTUP_HOME}
                                            export PATH="${RUSTUP_HOME}:$PATH"
                                        else
                                            echo "Rust is already installed"
                                        fi
                                        ${RUSTUP_HOME}/rustc --version
                                        ${RUSTUP_HOME}/cargo --version
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

