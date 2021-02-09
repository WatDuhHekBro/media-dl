import sys, subprocess, urllib.request, argparse
from urllib.parse import urlparse # test: https://stackoverflow.com/questions/66068134/segmentation-fault-when-using-a-shared-ptr-for-private-key?noredirect=1#comment116877826_66068134

# Check for updates and update if there is an update available

local_version = subprocess.run("youtube-dl --version", capture_output=True).stdout.decode(sys.stdout.encoding).strip()
remote_version = urllib.request.urlopen("http://rg3.github.io/youtube-dl/update/LATEST_VERSION").read().decode(sys.stdout.encoding).strip()

if local_version != remote_version:
	subprocess.run("youtube-dl -U")

# Check for args and prompt the user if there aren't any yet

args = sys.argv[1:]

if len(args) == 0:
	args = input("Flags: ").split(" ")

print(f"\"{args}\"")