locals {
  name_prefix = "${var.project_name}-production"
}

module "ssh_key" {
  source = "../../modules/digitalocean/ssh_key"

  name = "${local.name_prefix}-ssh-key"
}

module "droplet" {
  source = "../../modules/digitalocean/droplet"

  name     = "${local.name_prefix}-droplet"
  region   = var.region
  size     = var.size
  ssh_keys = [module.ssh_key.id]
  tags     = [var.project_name, "production"]
}

module "project" {
  source = "../../modules/digitalocean/project"

  name        = local.name_prefix
  description = "${var.project_name} production"
  resources   = [module.droplet.urn]
}
