use_module(library(lists)).

solve1(File, Passports, Valid) :-
  open(File, read, Stream),
  lazy_list(lazy_read_lines(Stream, []), List),
  parse(List, [], [], PassportsList),
  maplist(to_dict, PassportsList, Passports),
  include(valid, Passports, Valid),
  close(Stream).

to_dict(List, Dict) :- dict_pairs(Dict, pp, List).

solve2(File, Passports, Valid) :-
  open(File, read, Stream),
  lazy_list(lazy_read_lines(Stream, []), List),
  parse(List, [], [], PassportsList),
  maplist(to_dict, PassportsList, Passports),
  include(valid, Passports, HasFields),
  include(checkall, HasFields, Valid),
  close(Stream).

fields([byr, iyr, eyr, hgt, hcl, ecl, pid]).

valid(Passport) :-
  fields(Mandatory), 
  dict_keys(Passport, PKeys),
  intersection(PKeys, Mandatory, Isect),
  length(Isect, ValidFields),
  ValidFields is 7.

get_key(V, K) :- split_string(V, ":", " ", [KStr|_]), atom_string(K, KStr).

parse([], Rec, Acc, Matches) :-
  append([Rec], Acc, Matches).
parse([""|Rest], Rec, Acc, Matches) :-
  append([Rec], Acc, NewAcc),
  parse(Rest, [], NewAcc, Matches).
parse([Line|Rest], Rec, Acc, Matches) :-
  split_string(Line, " ", " ", KeyVals),
  map_list_to_pairs(get_key, KeyVals, Keys),
  append(Keys, Rec, NewRec),
  parse(Rest, NewRec, Acc, Matches).

% solve("simple.txt", P, V), length(V, Simple1).
% solve("input.txt", P, V), length(V, Part1).

% byr (Birth Year) - four digits; at least 1920 and at most 2002.
% iyr (Issue Year) - four digits; at least 2010 and at most 2020.
% eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
% hgt (Height) - a number followed by either cm or in:
%   If cm, the number must be at least 150 and at most 193.
%   If in, the number must be at least 59 and at most 76.
% hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
% ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
% pid (Passport ID) - a nine-digit number, including leading zeroes.
% cid (Country ID) - ignored, missing or not.


valid_height(H, "in") :- between(59, 76, H).
valid_height(H, "cm") :- between(150, 193, H).
check_height(Passport) :-
  F = Passport.hgt,
  re_matchsub("^hgt:(?<hgt_I>\\d+)(?<hgt_unit>cm|in)$", F, Sub, []),
  valid_height(Sub.hgt, Sub.hgt_unit).

check(Passport, K, Rule) :-
  F = Passport.K,
  re_matchsub("
    (^byr):(?<byr_I>\\d\\d\\d\\d)$
    | (^iyr):(?<iyr_I>\\d\\d\\d\\d)$
    | (^eyr):(?<eyr_I>\\d\\d\\d\\d)$
    | (^hcl):(?<hcl>\\#[0-9a-f]{6})$
    | (^ecl):(?<ecl>amb|blu|brn|gry|grn|hzl|oth)$
    | (^pid):(?<pid>\\d{9})$"/x, F, Sub, []),
    V = Sub.K,
    call(Rule, V).
  
any(_).
checkall(Passport) :-
  check(Passport, byr, between(1920, 2002)),
  check(Passport, iyr, between(2010, 2020)),
  check(Passport, eyr, between(2020, 2030)),
  check_height(Passport),
  check(Passport, hcl, any),
  check(Passport, ecl, any),
  check(Passport, pid, any).
