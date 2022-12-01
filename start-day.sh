: ${AOC_SESSION?"You need to set AOC_SESSION to your actual AoC 'session' cookie"}
# exit even if sourced
if [ "$AOC_SESSION" == "" ]; then return; fi
curr_day=`date +%-d`
year=`date +%Y`
day=${1:-$curr_day}
echo "Advent of Code - Day $day"
if [ ! -d aoc$day ]; then
    echo "Creating aoc$day folder ..."
    mkdir aoc$day
fi
FOLDER="$PWD/$(dirname $0)"
cd aoc$day

function checked_download() {
    if http --check-status --ignore-stdin --timeout=5 --output $1 $2 Cookie:session=$AOC_SESSION &> /dev/null; then
        echo "Download succeeded"
    else
        echo -n "Download $1 failed: "
        case $? in
            2) echo 'Request timed out!' ;;
            3) echo 'Unexpected HTTP 3xx Redirection!' ;;
            4) echo 'HTTP 4xx Client Error!' ;;
            5) echo 'HTTP 5xx Server Error!' ;;
            6) echo 'Exceeded --max-redirects=<n> redirects!' ;;
            *) echo 'Other Error!' ;;
        esac
        if [ -f $1 ]; then
            cat $1
            rm $1
        fi
    fi
}

BASE_URL="https://adventofcode.com/$year/day/$day"
if [ ! -f input.txt ]; then
    INPUT_URL="$BASE_URL/input"
    checked_download "input.txt" $INPUT_URL
fi

DAYHTML="day$day.html"
if [ ! -f $DAYHTML ]; then
    checked_download $DAYHTML $BASE_URL
fi

if [ ! -f simple.txt ] && [ -f $DAYHTML ]; then
    perl -MHTML::Entities $FOLDER/simple.pl < $DAYHTML > simple.txt
    if [ -f $DAYHTML ]; then
        rm $DAYHTML
    fi
fi

if [ -f simple.txt ]; then
    echo "simple.txt (full):"
    cat simple.txt
    echo ""
fi

if [ -f input.txt ]; then
    echo "input.txt (truncated):"
    head input.txt | cut -c -80
    echo -e "\n   lines   words   chars"
    wc input.txt
fi
echo -e "\nAll set!\n"
echo "See $BASE_URL for further instructions"
