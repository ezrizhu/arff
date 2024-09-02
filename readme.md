# Arff!

File uploader powered by Cloudflare workers, KV, and R2

Usage Examples

Upload

`curl -H 'Authorization: Bearer key' -F'file=@file.pdf' arf.sh`

`cat test | curl -H 'Authorization: Bearer key' -F'file=@-' arf.sh`

`curl -H 'Authorization: Bearer key' -F'file=@LICENSE;type=text/plain' arf.sh`

Delete

`curl -XDELETE -H 'Authorization: Bearer key' arf.sh/object_key`

Update

`curl -XPOST -H 'Authorization: Bearer key' -F'file=@file.pdf' arf.sh/object_key`

File extensions are ignored by the API.

Content-Type are saved used for downloads.

Account are created via a Cloudflare KV pair.

`Authentication: Bearer userID.secret`
