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

        stage('Checking Docker') {
            steps {
                    script {
                            // Ensure Rust is installed and available in the PATH
                            sh '''
                                echo "Checking for Docker installation..."

                                # Check if Docker service is active
                                if ! systemctl is-active --quiet docker; then
                                  echo "Docker daemon is not running."
                                  sudo service docker start

                                  while (! systemctl is-active --quiet docker); do
                                    # Docker takes a few seconds to initialize
                                    echo "Waiting for Docker to launch..."
                                    sleep 1
                                  done

                                else
                                  echo "Docker daemon is running."
                                fi

#                                 if ! [ -x "$(command -v docker)" ]; then
#                                     echo "Docker not found, installing..."
#                                    curl -fsSL https://get.docker.com | sh
#                                 fi
#                                sudo service docker start
                                docker --version
                                docker pull hello-world
                            '''
                            }
            }

        }
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

        stage('Create Docker version of image') {
            steps {
                script{
                    sh '''
                        echo "Creating a docker image..."
                        docker build -t rublevgeorgij/rust-api-test:v0.${BUILD_NUMBER} .
                        '''
                }

            }
        }
        stage('Create Docker latest version of image') {
            steps {
                script{
                    sh '''
                        echo "Creating a docker image..."
                        docker build -t rublevgeorgij/rust-api-test:latest .
                        '''
                }

            }
        }

        stage('Register Docker') {
                    steps {
                        script{
                            sh '''
                                docker login -u rublevgeorgij -p dckr_pat_NCO665FWIQQFxEuX-GkmqhBinZo
                                '''
                        }

                    }
                }

        stage('Push Docker version of image') {
            steps {
                script{
                    sh '''
                        docker push rublevgeorgij/rust-api-test:v0.${BUILD_NUMBER}
                        '''
                }

            }
        }
        stage('Push Docker latest version of image') {
            steps {
                script{
                    sh '''
                        docker push rublevgeorgij/rust-api-test:latest
                        '''
                }

            }
        }
    }
}

