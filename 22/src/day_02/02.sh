cat "../../input/22_02_test" | sed 's/ //g' | \
sed 's/X/X+1/g' | sed 's/Y/Y+2/g'| sed 's/Z/Z+3/g' | \
sed 's/AX\|BY\|CZ/3/g' | sed 's/AY\|BZ\|CX/6/g' | sed 's/AZ\|BX\|CY/0/g' | \
bc | sed -z 's/\n/+/g' | sed -z 's/.$/\n/' | bc

cat "../../input/22_02_test" | sed 's/ //g' | \
sed 's/Y/Y+3/g'| sed 's/Z/Z+6/g' | \
sed 's/AX\|BZ\|CY/3/g' | sed 's/AY\|BX\|CZ/1/g' | sed 's/AZ\|BY\|CX/2/g' | \
bc | sed -z 's/\n/+/g' | sed -z 's/.$/\n/' | bc
