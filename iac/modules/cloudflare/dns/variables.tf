variable "dns_root_zone" {
  type = string
}

variable "server_ip" {
  type = string
}

variable "subdomains" {
  type = list(string)
}

variable "ttl" {
  type    = number
  default = 300
}

variable "proxied" {
  type    = bool
  default = false
}
