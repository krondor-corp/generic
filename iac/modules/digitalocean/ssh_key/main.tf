resource "tls_private_key" "main" {
  algorithm = "ED25519"
}

resource "digitalocean_ssh_key" "main" {
  name       = var.name
  public_key = tls_private_key.main.public_key_openssh
}
