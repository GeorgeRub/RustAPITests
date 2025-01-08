pipeline {
    agent {
        label 'agent'
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