# Infra

Terraform skeleton for Project Berean's GCP footprint: API enablement,
AlloyDB (relational + vector), the three Vertex AI Search datastores
(corpus / commentary / skeptic-scholarship — matches the agent tooling in
`../../agents`), and a Cloud Run service placeholder for the Berean Engine.

## Deliberately out of scope here

- **AlloyDB networking prerequisites** (VPC + private services access) —
  environment-specific; create the network and pass its self-link as
  `network_id`. See
  https://cloud.google.com/alloydb/docs/configure-connectivity.
- **Agent Engine deployment** — as of this writing there isn't full
  first-class Terraform coverage for deploying ADK agents to Agent Engine;
  that's done via the ADK CLI/SDK (`adk deploy agent_engine`, run from
  `../../agents`) against the project/region provisioned here.
- **Corpus ingestion** — datastores are created empty; loading translations,
  commentary, and lexicon data is separate work.

## Usage

```bash
terraform init
terraform plan  -var="project_id=..." -var="network_id=..." -var="alloydb_password=..."
terraform apply -var="project_id=..." -var="network_id=..." -var="alloydb_password=..."
```

Prefer a `terraform.tfvars` (gitignored) or `TF_VAR_*` env vars over passing
`alloydb_password` on the command line where shell history persists.
