output "zone_id" {
  value = data.cloudflare_zone.main.id
}

output "records" {
  value = {
    for slug, record in cloudflare_record.subdomains : slug => {
      id       = record.id
      hostname = record.hostname
    }
  }
}
