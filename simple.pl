while (<>) {
    # one-line codes
    if (/.pre..code.(.*)..code...pre.$/) {
        print decode_entities("$1\n");
    }

    # end of any code
    if (/..code...pre/) {
        exit(0);
    }

    # continuation of multi-line codes
    if ($simple)
    {
        print decode_entities($_);
    }

    # start of multi-line codes
    if (/.pre..code.(.*)$/) {
        print decode_entities("$1\n");
        $simple = true;
    }
}
