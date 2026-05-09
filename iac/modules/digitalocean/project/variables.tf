variable "name" {
  type = string
}

variable "description" {
  type    = string
  default = ""
}

variable "environment" {
  type    = string
  default = "Production"
}

variable "resources" {
  type    = list(string)
  default = []
}
