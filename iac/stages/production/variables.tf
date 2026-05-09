variable "project_name" {
  type = string
}

variable "root_public_key" {
  type        = string
  description = "SSH public key for initial root access (from 1Password)"
}

variable "region" {
  type    = string
  default = "nyc3"
}

variable "size" {
  type    = string
  default = "s-2vcpu-4gb"
}
