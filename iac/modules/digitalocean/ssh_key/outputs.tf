output "id" {
  value = digitalocean_ssh_key.main.id
}

output "fingerprint" {
  value = digitalocean_ssh_key.main.fingerprint
}

output "private_key_openssh" {
  value     = tls_private_key.main.private_key_openssh
  sensitive = true
}

output "public_key_openssh" {
  value = tls_private_key.main.public_key_openssh
}
