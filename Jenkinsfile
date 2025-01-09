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

        stage('Setup Rust') {
            steps {
                    script {
                            // Ensure Rust is installed and available in the PATH
                            sh '''
                                echo "Checking for Rust installation..."
                                if ! [ -x "$(command -v rustc)" ]; then
                                    echo "Rust not found, installing..."
                                    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
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
        stage('Build') {
            steps {
                script{
                    sh 'cargo build --release' // Build the Rust application in release mode
                }

            }
        }
        stage('Checking Docker') {
            steps {
                    script {
                            // Ensure Rust is installed and available in the PATH
                            sh '''
                                echo "Checking for Docker installation..."
                                docker pull hello-world
                            '''
                            }
            }

        }
        stage('Create Docker image') {
            steps {
                script{
                    sh '''
                        echo "Creating a docker image..."
                        docker build -t rublevgeorgij/rust-api-test:v0.${BUILD_NUMBER} .
                        docker login -u rublevgeorgij -p dckr_pat_NCO665FWIQQFxEuX-GkmqhBinZo
                        '''
                }

            }
        }
    }
}

