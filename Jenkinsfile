pipeline {
    agent {
        label 'agent1'
      }
    stages {

        stage('Build') {
          steps {
          sh "cargo version"
            sh "cargo build --release"
          }
          
        }
      }
}