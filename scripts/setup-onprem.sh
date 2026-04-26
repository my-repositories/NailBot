#!/usr/bin/env bash
set -euo pipefail

ENV_FILE="${1:-.env}"

if [[ ! -f "${ENV_FILE}" ]]; then
  cp .env.onprem.example "${ENV_FILE}"
  echo "Created ${ENV_FILE} from .env.onprem.example"
fi

content="$(cat "${ENV_FILE}")"
required=(
  "MODE=onprem"
  "BOT_TOKEN="
  "ADMIN_IDS="
  "DATABASE_URL="
  "LICENSE_KEY="
)

for entry in "${required[@]}"; do
  if ! grep -Fq "${entry}" <<<"${content}"; then
    echo "Missing required setting marker: ${entry}" >&2
    exit 1
  fi
done

docker compose --env-file "${ENV_FILE}" up -d --build

echo
echo "On-prem setup complete."
echo "Verification:"
echo "1) docker compose ps"
echo "2) docker compose logs -f api bot"
echo "3) Send /start to the bot"
