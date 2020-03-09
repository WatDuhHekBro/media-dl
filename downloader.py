import os

flags = {
	'video': '-f bestvideo+bestaudio/best',
	'audio': '-x --audio-format mp3',
	'hq': '--audio-quality 0',
	'subs': '--all-subs',
	'desc': '--write-description',
	'thumb': '--write-all-thumbnails --embed-thumbnail'
}
presets = {
	# youtube-dl -f bestvideo+bestaudio/best 
	'video': flags['video'],
	# youtube-dl -f bestvideo+bestaudio/best --all-subs --write-description --write-all-thumbnails --embed-thumbnail 
	'videoext': '{} {} {} {}'.format(flags['video'], flags['subs'], flags['desc'], flags['thumb']),
	# youtube-dl -x --audio-format mp3 
	'audio': flags['audio'],
	# youtube-dl -x --audio-format mp3 --audio-quality 0 
	'audiohq': '{} {}'.format(flags['audio'], flags['hq'])
}

url = input("Input URLs (multiple URLs separated by spaces): ")

if not url:
	print("WARNING: URLs are empty! Exiting program.")
else:
	url = url.split(' ')
	print("Counting " + str(len(url)) + " URL(s).")
	go = False
	both = False
	
	args = input("\nEnter one of the following presets or input flags:\n- video\n- videoext\n- audio\n- audiohq\n- flags\n- both\nPreset: ")
	
	if args == 'flags':
		f = input("\nNext, enter any flags:\n- video\n- audio\n- hq\n- subs\n- desc\n- thumb\nSpace Separated Flags: ").split(' ')
		args = ''
		
		for x in f:
			if x in flags:
				args += ' ' + flags[x]
		
		go = True
	elif args == 'both':
		args = ''
		video = presets['videoext'] if input("\nDo you want extended video data? (Leave blank for standard video data) [ext/st] ") == 'ext' else presets['video']
		audio = presets['audio'] if input("Do you want HQ audio or standard audio? (Leave blank for HQ audio) [hq/st] ") == 'st' else presets['audiohq']
		go = True
		both = True
	elif args in presets:
		args = presets[args]
		go = True
	else:
		print("\nThe preset you entered is not valid. Exiting program.")
	
	if go:
		if input("\nAre these playlists URLs? (Leave blank for \"no\") [y/n] ").lower() == 'y':
			args += ' -o "%(playlist)s/%(playlist_index)s - %(title)s.%(ext)s"'
		else:
			d = input("Custom directory? (Leave blank for \"General\") ")
			d = d if d else 'General'
			args += ' -o "{}/%(title)s.%(ext)s"'.format(d)
		print()
		
		args = args.lstrip()
		
		if both:
			for u in url:
				os.system('youtube-dl {} {} {}'.format(audio, args, u))
				print()
				os.system('youtube-dl {} {} {}'.format(video, args, u))
				print()
		else:
			for u in url:
				os.system('youtube-dl {} {}'.format(args, u))
				print()
	# EOF
# Add error msg without using cmd.exe?
input("\nPress any key to continue...")