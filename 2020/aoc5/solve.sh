tr FBRL 0110 < input.txt | sort | sed $'1i\\\nibase=2;' | bc > seats.txt
LAST=`tail -1 seats.txt`
FIRST=`head -1 seats.txt`
echo $LAST
seq $FIRST $LAST >> seats.txt
sort -n seats.txt | uniq -u
