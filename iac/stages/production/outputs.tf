output "server_ip" {
  value = module.droplet.ipv4_address
}

output "ssh_private_key" {
  value     = module.ssh_key.private_key_openssh
  sensitive = true
}

output "ssh_public_key" {
  value = module.ssh_key.public_key_openssh
}
