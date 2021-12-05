while (<>) {
    if (/..code...pre/) {
        exit(0);
    }

    if ($simple)
    {
        print decode_entities($_);
    }

    if (/.pre..code.(.*)$/) {
        print decode_entities("$1\n");
        $simple = true;
    }
}
