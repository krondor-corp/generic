variable "project_name" {
  type = string
}

variable "dns_root_zone" {
  type = string
}

variable "subdomains" {
  type = list(string)
}

variable "region" {
  type    = string
  default = "nyc3"
}

variable "size" {
  type    = string
  default = "s-2vcpu-4gb"
}
