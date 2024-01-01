$sampleIndex = $ENV{'AOC_SAMPLE_INDEX'} || 0; 
while (<>) {
    $active = $sampleIndex == 0;
    # one-line codes
    if ($active && /.pre..code.(.*)..code...pre.$/) {
        print decode_entities("$1\n");
    }
    # end of any code
    if (/..code...pre/) {
        if ($sampleIndex < 1) {
            exit(0);
        } else {
            $sampleIndex--;
        }
    }

    # continuation of multi-line codes
    if ($simple)
    {
        print decode_entities($_);
    }

    # start of multi-line codes
    if ($active && /.pre..code.(.*)$/) {
        print decode_entities("$1\n");
        $simple = true;
    }
}
