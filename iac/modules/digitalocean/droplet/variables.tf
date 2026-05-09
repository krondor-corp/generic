variable "name" {
  type = string
}

variable "region" {
  type    = string
  default = "nyc3"
}

variable "size" {
  type    = string
  default = "s-1vcpu-1gb"
}

variable "image" {
  type    = string
  default = "ubuntu-24-04-x64"
}

variable "ssh_keys" {
  type    = list(string)
  default = []
}

variable "tags" {
  type    = list(string)
  default = []
}
