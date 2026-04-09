group "default" {
  targets = ["mentaldesk"]
}

target "mentaldesk" {
  context = "."
  dockerfile = "Dockerfile"
  tags = ["mediocrehacker/mentaldesk:latest"]
}
