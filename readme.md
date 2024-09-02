# Arff!

File uploader powered by Cloudflare workers, KV, and R2

Usage Examples

`curl -H 'Authorization: Bearer key' -F'file=@file.pdf' arf.sh`

`cat test | curl -H 'Authorization: Bearer key' -F'file=@-' arf.sh`

`curl -H 'Authorization: Bearer key' -F'file=@LICENSE;type=text/plain' arf.sh`

Account are created via a Cloudflare KV pair.

`Authentication: Bearer userID.secret`
