WIDE=$2
TALL=$3
let LAYER_SIZE="($WIDE + 1) * $TALL"
NAME="out-$WIDE-$TALL.txt"
echo "Creating $NAME"
# wrap lines to WIDE
fold -$WIDE $1 > $NAME
rm -r layers/
mkdir -p layers/ppm
mkdir -p layers/png
# split into layers
split -b$LAYER_SIZE $NAME layers/layer-
command -v rg &> /dev/null
if [ $? != 0 ]; then
    echo "needs rg"
    exit
fi

# find the layer that contains the fewest 0 digits
LAYER=`rg --count-matches 0 layers/layer-* | sort --field-separator=: --key=2 --numeric | head -1 | cut -d: -f1`
echo "Fewest 0s in $LAYER"
# On that layer, what is the number of 1 digits multiplied by the number of 2 digits?
ONES=`rg --count-matches 1 $LAYER || echo 0`
TWOS=`rg --count-matches 2 $LAYER || echo 0`
let ANSWER="$ONES * $TWOS"
echo "Answer Part1: $ONES * $TWOS = $ANSWER"

# convert to PPM (text-based image format),
# Textual Grayscale (P2 format) with white=2 (grayscale level 0-2)
# 0 = black, 1 = gray, 2 = white
HEAD="1i\\\\\nP2\\\\\n$WIDE $TALL\\\\\n2\\\\\n"
find layers/ -name 'layer-*' -exec sh -c "sed -e $'$HEAD' -e 's/./& /g' '{}' >> '{}.ppm'" \;

# check for imagemagick
command -v magick &> /dev/null
if [ $? != 0 ]; then
    echo "needs magick"
    exit
fi

# convert to png, mapping white -> transparent, and gray -> white
find layers/ -name 'layer-*.ppm' -exec sh -c 'magick convert $1 -transparent white -fill white -fuzz 1% -opaque "#808080" "${1%.ppm}.png"' sh {} ';'

# combine the image layer by layer
magick convert -size "${WIDE}x${TALL}" xc:red composite.png
find layers -name 'layer-*.png'  -print0 | sort -rz | xargs -n1 -0 -I% magick composite '%' composite.png composite.png

# clean up
rm layers/*.ppm
rm layers/*.png
open composite.png
