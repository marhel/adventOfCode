#!/usr/local/bin/pwsh
function normalize($word) {
    -join($word.ToCharArray() | sort)
}

$v = $Input | Where-Object {
    $words = $_.Split(" ") | % { normalize($_) }
    $wcount = $words.Length 
    $uniq = $words | sort | uniq
    $ucount = $uniq.Length

    $wcount -eq $ucount
} | Measure-Object

$v.Count

