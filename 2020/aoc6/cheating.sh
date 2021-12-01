perl -p -e 's/([a-z])\n/\1/g' simple.txt | perl -lne 'chomp($_); print (join "", sort split //,$_)' | perl -e 's/([a-z])\1+/\1/g' | wc
