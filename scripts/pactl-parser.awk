# pactl list sink-inputs
BEGIN { RS = "\n\n"; FS = "\n"}
{
    split($1, tmp, " ")
    id = substr(tmp[3], 2)
    for (i = 2; i < NF; i++) {
	split($i, tmp, " ")
	switch ($i) {
	    case /media\.role/:
		if (tmp[3] == "\"filter\"") next; break
	    case /application\.name/:
		name = substr(tmp[3], 2, length(tmp[3]) - 2)
		break
	    case /Mute:/:
		mute = tmp[2]
		break
	    case /Volume:/:
		volume = int(tmp[3])
		break
	}
    }
    print id, name, volume, mute
}
