data "cloudflare_zone" "main" {
  name = var.dns_root_zone
}

resource "cloudflare_record" "subdomains" {
  for_each = toset(var.subdomains)

  zone_id = data.cloudflare_zone.main.id
  name    = each.value
  content = var.server_ip
  type    = "A"
  ttl     = var.ttl
  proxied = var.proxied
}
