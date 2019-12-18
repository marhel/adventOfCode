WIDE=$2
TALL=$3
LAYER_SIZE=`dc -e "$WIDE 1 + $TALL * p"` # Note: dc uses RPN
NAME="out-$WIDE-$TALL.txt"
echo "Creating $NAME"
# wrap lines to WIDE
fold -$WIDE $1 > $NAME
# split into layers
split -b$LAYER_SIZE $NAME layer-
command -v rg &> /dev/null
if [ $? != 0 ]; then
    echo "needs rg"
    exit
fi

# find the layer that contains the fewest 0 digits
LAYER=`rg --count-matches 0 layer-* | sort --field-separator=: --key=2 --numeric | head -1 | cut -d: -f1`
echo "Fewest 0s in $LAYER"
# On that layer, what is the number of 1 digits multiplied by the number of 2 digits?
ONES=`rg --count-matches 1 $LAYER`
TWOS=`rg --count-matches 2 $LAYER`
ANSWER=`dc -e "$ONES $TWOS * p"` # Note: dc uses RPN
echo "Answer Part1: $ONES * $TWOS = $ANSWER"
