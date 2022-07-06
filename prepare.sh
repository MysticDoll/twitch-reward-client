#/bin/bash

echo access "https://id.twitch.tv/oauth2/authorize?client_id=${TWITCH_CLIENT_ID}&redirect_uri=http://localhost:8000/twark/&response_type=code&scope=chat:read+chat:edit+channel:read:redemptions"

CODE=$(nc -lp8000 | head -1 | cut -d'?' -f2 | cut -d'=' -f2 | cut -d'&' -f1)
export TWITCH_ACCESS_TOKEN=$(curl -XPOST "https://id.twitch.tv/oauth2/token?client_id=${TWITCH_CLIENT_ID}&client_secret=${TWITCH_CLIENT_SECRET}&code=${CODE}&grant_type=authorization_code&redirect_uri=http://localhost:8000/twark/" | jq -r '.access_token')
