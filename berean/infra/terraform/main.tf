terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 6.0"
    }
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}

locals {
  apis = [
    "aiplatform.googleapis.com",       # Gemini Enterprise Agent Platform (Agent Engine, models)
    "discoveryengine.googleapis.com",  # Vertex AI Search (RAG datastores)
    "alloydb.googleapis.com",
    "run.googleapis.com",
    "artifactregistry.googleapis.com",
  ]
}

resource "google_project_service" "apis" {
  for_each           = toset(local.apis)
  project            = var.project_id
  service            = each.value
  disable_on_destroy = false
}

# --- Data layer -------------------------------------------------------------
# One AlloyDB cluster/instance for relational data + embeddings
# (users, study plans, group threads, curated cross-ref edges).

resource "google_alloydb_cluster" "berean" {
  cluster_id = "berean-primary"
  location   = var.region
  network_config {
    network = var.network_id
  }
  initial_user {
    user     = "berean"
    password = var.alloydb_password
  }
  depends_on = [google_project_service.apis]
}

resource "google_alloydb_instance" "berean_primary" {
  cluster       = google_alloydb_cluster.berean.name
  instance_id   = "berean-primary-instance"
  instance_type = "PRIMARY"
  machine_config {
    cpu_count = 2
  }
}

# --- Grounding datastores -----------------------------------------------
# Tradition-tagged so whole_counsel_agent can label consensus by counting
# which tagged sources agree. See docs/ARCHITECTURE.md.

resource "google_discovery_engine_data_store" "corpus" {
  for_each          = toset(["corpus", "commentary", "skeptic-scholarship"])
  project           = var.project_id
  location          = "global"
  data_store_id     = "berean-${each.value}"
  display_name      = "Berean ${each.value}"
  industry_vertical = "GENERIC"
  solution_types    = ["SOLUTION_TYPE_SEARCH"]
  content_config    = "CONTENT_REQUIRED"
  depends_on        = [google_project_service.apis]
}

# --- Berean Engine (Rust MCP server) ----------------------------------------
# Built and pushed to Artifact Registry by CI; this just declares the service.
# Image left as a placeholder until the first image is published.

resource "google_artifact_registry_repository" "berean" {
  location      = var.region
  repository_id = "berean"
  format        = "DOCKER"
  depends_on    = [google_project_service.apis]
}

resource "google_cloud_run_v2_service" "berean_engine" {
  name     = "berean-engine"
  location = var.region
  deletion_protection = false

  template {
    containers {
      image = "${var.region}-docker.pkg.dev/${var.project_id}/berean/berean-engine:latest"
    }
  }

  depends_on = [google_artifact_registry_repository.berean]
}

output "alloydb_cluster" {
  value = google_alloydb_cluster.berean.name
}

output "datastore_ids" {
  value = { for k, v in google_discovery_engine_data_store.corpus : k => v.data_store_id }
}
