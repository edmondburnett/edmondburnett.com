#!/usr/bin/env zsh
set -euo pipefail

ENV="${1:-local}"

if [[ "$ENV" != "local" && "$ENV" != "prod" ]]; then
    echo "usage: $0 [local|prod]   (default: local)" >&2
    exit 1
fi

DEST=/srv/http/edmondburnett.com
REMOTE=moon
SERVICE=edmondburnett-com.service
ASSET_DIRS=(pages posts projects static)

cargo test
cargo run --release -- validate

# Render the runtime env file that the systemd unit reads (EnvironmentFile).
ENV_FILE="$(mktemp)"
trap 'rm -f "$ENV_FILE"' EXIT
printf 'CURRENT_ENV=%s\n' "$ENV" > "$ENV_FILE"
echo "==> CURRENT_ENV=$ENV"

if [[ "$ENV" == "local" ]]; then
    echo "==> deploying locally to $DEST"

    sudo systemctl stop "$SERVICE"

    sudo cp conf/systemd/"$SERVICE" "$DEST"
    sudo cp target/release/edmondburnett-com "$DEST"
    sudo cp "$ENV_FILE" "$DEST/.env"

    for dir in $ASSET_DIRS; do
        sudo rm -rf "$DEST/$dir"
        sudo cp -r "$dir" "$DEST"
    done

    sudo chown -R edmondburnett-com:edmondburnett-com "$DEST"
    sudo systemctl link "$DEST/$SERVICE"
    sudo systemctl daemon-reload
    sudo systemctl restart "$SERVICE"
else
    echo "==> deploying to prod ($REMOTE:$DEST)"

    # Relative to the remote home, so it works regardless of differing home paths.
    STAGE=dev/edmondburnett-com-deploy-staging
    STAGE_WEB="$STAGE/web"        # binary + asset dirs -> $DEST
    STAGE_NGINX="$STAGE/nginx"    # nginx config -> /etc/nginx/sites-available

    # Transfer into staging as our own user (no sudo, no TTY needed).
    rsync -az --mkpath target/release/edmondburnett-com conf/systemd/"$SERVICE" "$REMOTE:$STAGE_WEB/"
    rsync -az "$ENV_FILE" "$REMOTE:$STAGE_WEB/.env"

    for dir in $ASSET_DIRS; do
        rsync -az --delete --mkpath "$dir" "$REMOTE:$STAGE_WEB/"
    done

    rsync -az --mkpath conf/nginx/edmondburnett.com "$REMOTE:$STAGE_NGINX/"

    # One interactive session: sudo prompts once, then caches for the rest.
    ssh -t "$REMOTE" "
        set -e
        sudo systemctl stop $SERVICE
        sudo rsync -a --delete $STAGE_WEB/ $DEST/
        sudo cp $STAGE_NGINX/edmondburnett.com /etc/nginx/sites-available/edmondburnett.com
        sudo chown -R edmondburnett-com:edmondburnett-com $DEST
        sudo find /var/cache/nginx-edmondburnett-com -mindepth 1 -delete
        sudo systemctl link $DEST/$SERVICE
        sudo systemctl daemon-reload
        sudo systemctl reload nginx
        sudo systemctl restart $SERVICE
    "
fi

echo "==> $ENV deploy complete"
