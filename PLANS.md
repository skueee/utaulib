If I need to plan something for long things I'll do it here.

Checkout the Git history if ya want to see some thoughts idk.

# Storing voices

I already have a voicebank structure that stores the character info, so voices will be in there.

I just need a way to set if they're loaded or not, and to store the audio files. This could be done with enums but I'm lazy.

I need to store a map of every syllabe, tone and stuff like that, so it can be selected, then when the user tries to render I check if it is loaded and throw an error if it is not. Why the heck am I speaking in I idk. So the program needs to keep a trace of every voice, with nicknames and stuff so that the user can load them easily not to get an error.

I'll use an enum then...

## Mapping

Idk yet for that, I still need to read shit about UTAU and stuff bro

## Storing

Voices will be stored in structs will all the voices infos (a name for it, stuff like that) and an enum that will either be empty (if the voice is not loaded) or will contain the voice audio data. It will have some functions to easily be loaded/unloaded for performaaaaaance. If the voice is not loaded, it will throw an error asking to load the voice.
