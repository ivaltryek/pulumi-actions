FROM ghcr.io/lowkey-who/pulumi-actions:latest

COPY entrypoint.sh /entrypoint.sh

# github action recommendation is to run as root.
# https://docs.github.com/en/actions/creating-actions/dockerfile-support-for-github-actions#user
USER root

ENTRYPOINT ["/entrypoint.sh"]