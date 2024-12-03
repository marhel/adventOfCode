% this works, but isn't left recursive
old_char_count([], c{}).
old_char_count([Char|Rest], Dict) :-
  old_char_count(Rest, Partial),
  get_dict(Char, Partial, CurrentCount),
  NewCount is CurrentCount + 1,
  Dict = Partial.put(Char, NewCount).
old_char_count([Char|Rest], Dict) :-
  old_char_count(Rest, Partial),
  \+ get_dict(Char, Partial, _),
  Dict = Partial.put(Char, 1).

% this is left recursive
char_count(L, D) :- char_count_acc(L, c{}, D).
char_count_acc([], Dict, Dict).
char_count_acc([Char|Rest], Accum, Dict) :-
  get_dict(Char, Accum, CurrentCount), !,
  NewCount is CurrentCount + 1,
  char_count_acc(Rest, Accum.put(Char, NewCount), Dict).
char_count_acc([Char|Rest], Accum, Dict) :-
  char_count_acc(Rest, Accum.put(Char, 1), Dict).

solve(File, Rule, Matches) :-
  open(File, read, Stream),
  lazy_list(lazy_read_lines(Stream, []), List),
  include(Rule, List, Matches),
  close(Stream).

password_rule1(Line) :-
  parse(Line, Min, Max, Char, Password),
  string_chars(Password, PChars),
  char_count(PChars, Counts),
  % it is never ok with 0 occurrences, I've checked the input
  get_dict(Char, Counts, Count), 
  between(Min, Max, Count).

and(A,B) :- A,B.
or(A,B) :- A;B.
nand(A,B) :- not(and(A,B)).
xor(A,B) :- or(A,B), nand(A,B).

password_rule2(Line) :-
  parse(Line, Min, Max, Char, Password),
  string_chars(Password, PChars),
  A = nth1(Min, PChars, Char),
  B = nth1(Max, PChars, Char),
  xor(A, B).

parse(Line, Min, Max, CharAtom, Password) :-
  split_string(Line, ":", " ", [Req, Password]),
  split_string(Req, " ", " ", [Count, Char]),
  split_string(Count, "-", " ", [MinC, MaxC]),
  number_string(Min, MinC),
  number_string(Max, MaxC),
  atom_string(CharAtom, Char).

% solve("input.txt", password_rule1, Matches), length(Matches, Part1).
% solve("input.txt", password_rule2, Matches), length(Matches, Part2).