resource "digitalocean_droplet" "main" {
  name     = var.name
  region   = var.region
  size     = var.size
  image    = var.image

  ssh_keys = var.ssh_keys

  tags = concat(var.tags, ["managed-by-terraform"])

  monitoring        = true
  graceful_shutdown = false
}

resource "digitalocean_firewall" "main" {
  name = "${var.name}-firewall"

  droplet_ids = [digitalocean_droplet.main.id]

  inbound_rule {
    protocol         = "tcp"
    port_range       = "22"
    source_addresses = ["0.0.0.0/0", "::/0"]
  }

  inbound_rule {
    protocol         = "tcp"
    port_range       = "80"
    source_addresses = ["0.0.0.0/0", "::/0"]
  }

  inbound_rule {
    protocol         = "tcp"
    port_range       = "443"
    source_addresses = ["0.0.0.0/0", "::/0"]
  }

  outbound_rule {
    protocol              = "tcp"
    port_range            = "1-65535"
    destination_addresses = ["0.0.0.0/0", "::/0"]
  }

  outbound_rule {
    protocol              = "udp"
    port_range            = "1-65535"
    destination_addresses = ["0.0.0.0/0", "::/0"]
  }

  outbound_rule {
    protocol              = "icmp"
    destination_addresses = ["0.0.0.0/0", "::/0"]
  }
}
