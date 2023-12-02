import json
import re
from http import client
from pathlib import Path
from urllib.parse import urlparse
from zipfile import ZipFile

DEPS = Path(__file__).parent.parent.joinpath("deps")
DEPS.mkdir(exist_ok=True)

conn = client.HTTPSConnection("api.github.com")
conn.request("GET", "/repos/protocolbuffers/protobuf/releases", headers={
    "Accept": "application/vnd.github+json",
    "X-GitHub-Api-Version": "2022-11-28",
    "User-Agent": "CI/CD",
})
resp = conn.getresponse()
last_release = json.loads(resp.read())[0]
print("Found",last_release["name"])
for asset in last_release["assets"]:
    if re.match(r'protoc-.*-win64.zip', asset["name"]):
        break
else:
    raise RuntimeError("not found")

print("Downloading", asset["name"], "from", asset["browser_download_url"])
target = asset["browser_download_url"]
while target:
    print(target)
    url = urlparse(target)
    if url.scheme == "https":
        conn = client.HTTPSConnection(url.hostname, url.port)
    elif url.scheme == "http":
        conn = client.HTTPConnection(url.hostname, url.port)
    conn.request("GET", f'{url.path}?{url.query}')
    resp = conn.getresponse()
    if resp.status // 100 == 3:
        target = resp.getheader("Location")
    else:
        break
    
assert resp.status == 200, resp.status

protoc_zip = DEPS.joinpath(asset["name"])
with protoc_zip.open("wb") as fp:
    for chunk in iter(lambda: resp.read(1024), b''):
        fp.write(chunk)

ZipFile(protoc_zip).extractall(DEPS)