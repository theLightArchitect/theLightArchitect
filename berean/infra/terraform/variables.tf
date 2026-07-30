variable "project_id" {
  description = "GCP project ID for Project Berean."
  type        = string
}

variable "region" {
  description = "Primary region for Cloud Run, AlloyDB, and datastores."
  type        = string
  default     = "us-central1"
}

variable "network_id" {
  description = <<-EOT
    Self-link of an existing VPC network with private services access
    configured for AlloyDB (see the VPC peering step in
    https://cloud.google.com/alloydb/docs/configure-connectivity). Not
    created here — AlloyDB's networking prerequisites are environment-specific
    enough that they're deliberately out of scope for this scaffold.
  EOT
  type        = string
}

variable "alloydb_password" {
  description = "Password for the AlloyDB initial user. Pass via -var or TF_VAR_alloydb_password, never commit it."
  type        = string
  sensitive   = true
}
