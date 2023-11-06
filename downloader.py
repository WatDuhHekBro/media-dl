import os
import re

# Match /watch?v= URLs and allow using links from alternate YouTube frontends directly
# ".*" is needed at the end in order to not have the match be None in the case of "https://www.youtube.com/watch?app=desktop&v=iHfJRON3b-w&feature=youtu.be"
ytvideo_pattern = re.compile(r'https:\/\/.+?\/watch\?(?:\w+=.+?&)*(v=.{11}).*')
# Do the same for YouTube playlists
playlist_pattern = re.compile(r'https:\/\/.+?(\/playlist\?list=.+)')

flags = {
	'video': '-f bestvideo+bestaudio/best',
	'all-fragments': '--abort-on-unavailable-fragment',
	'novideo': '--skip-download',
	'keep': '-k',
	'audio': '-x --audio-format mp3',
	'hq': '--audio-quality 0',
	'subs': '--all-subs',
	'desc': '--write-description',
	'thumb': '--write-all-thumbnails',
	'thumb-embed': '--embed-thumbnail'
}

url = input("Input URLs (space-separated): ")

if not url:
	print("ERROR: URLs are empty! Exiting program.")
else:
	url = url.split(' ')
	print("Counting " + str(len(url)) + " URL(s).")
	args = []

	# List flags
	print("\nAvailable Flags:")

	for key in flags.keys():
		print("- {}".format(key))

	# Get flags
	f = input("\nFlags (space-separated): ").split(' ')

	for x in f:
		if x in flags:
			args.append(flags[x])

	# Ask for output location
	d = input("Custom directory? (Default: \"general\") ")
	d = d if d else 'general'
	output = '-o "{}/%(title)s.%(ext)s"'.format(d)
	print()

	for u in url:
		# Detect if it's a YouTube video URL (but not necessarily on YouTube)
		match = re.fullmatch(ytvideo_pattern, u)
		#print(f'After matching video regex: {u} + {match}')

		if(match):
			u = 'https://www.youtube.com/watch?' + match.groups()[0]

		match = re.fullmatch(playlist_pattern, u)
		#print(f'After matching playlist regex: {u} + {match}')

		# Detect if it's a playlist URL and handle accordingly, ignore existing output location
		if(match):
			u = 'https://www.youtube.com/watch?' + match.groups()[0]
			print('Using Playlist URL: {}'.format(u))
			os.system('youtube-dl {} {} {}'.format(' '.join(args), '-o "%(playlist)s/%(playlist_index)s - %(title)s.%(ext)s"', u))
		else:
			print('Using Video URL: {}'.format(u))
			os.system('youtube-dl {} {} {}'.format(' '.join(args), output, u))
		print()

input("\nPress any key to continue...")
