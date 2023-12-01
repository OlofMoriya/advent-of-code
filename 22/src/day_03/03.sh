cat "../../input/22_03_test" | awk '
BEGIN {
  # Make this string for looking up all ascii characters (1-255):
  C = "" ; for ( i = 0 ; ++i < 256 ; ) C = C sprintf ( "%c" , i )
}
{
	a = substr($0, 0, length($0)/2)
	b = substr($0, length($0)/2+1, length($0))
	split(a, a_chars, "")
	found = ""
	out = ""
	for (i=1; i <= length(a); i++) {
		if (index(b, a_chars[i]) != 0 && index(found, a_chars[i]) == 0) {
			v = 0
			if (a_chars[i] >= "a") v = index(C,a_chars[i]) - index(C, "a") + 1 
			else v = index(C, a_chars[i]) - index(C, "A") + 27
			found = found a_chars[i]
			out = out v "+"
		}
	}
	
	printf("%s", out)
}' | sed -z 's/.$/\n/g' | bc

