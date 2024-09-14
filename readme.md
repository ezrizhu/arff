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

---

### ShareX Custom Uploader

If you use ShareX and wish to use arf.sh, you may do so by using this Custom Uploader preset: https://arf.sh/06afe7.sxcu

Make sure to change the marked value to include your authorization token.

<img src="https://arf.sh/c5589a.png" alt="Change this" width="800">
