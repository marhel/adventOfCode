lines(File, List, Stream) :-
  open(File, read, Stream),
  lazy_list(lazy_read_lines(Stream, []), List).

solve(List, Rule, Matches) :-
  index(List, 0, [], [_|Indexed]),
  include(Rule, Indexed, Matches).

hit_tree(Right, Rule, [Index, Trees]) :-
  string_chars(Trees, Chars),
  length(Chars, Mod),
  Pos is truncate(Index * Right) mod Mod,
  call(Rule, Index),
  nth0(Pos, Chars, #).

even(J) :- R is J mod 2, R is 0.
odd(J) :- R is J mod 2, R is 1.
positive(J) :- J > 0.

index([], _, LRes, LRes).
index([H|T], I, Acc, LRes) :-
  append(Acc, [[I,H]], NewAcc),
  J is I + 1,
  index(T, J, NewAcc, LRes).

part1(File, Length) :-
  lines(File, Lines, Stream),
  solve(Lines, hit_tree(3, positive), Results), 
  length(Results, Length), 
  close(Stream).

part1(File, Length) :-
  lines(File, Lines, Stream),
  solve(Lines, hit_tree(3, positive), Results), 
  length(Results, Length), 
  close(Stream).

mult(X, Y, Prod) :- Prod is X * Y.

part2(File, Lengths, Mul) :-
  lines(File, Lines, Stream),
  solve(Lines, hit_tree(1, positive), Results1), 
  solve(Lines, hit_tree(3, positive), Results2), 
  solve(Lines, hit_tree(5, positive), Results3), 
  solve(Lines, hit_tree(7, positive), Results4), 
  solve(Lines, hit_tree(0.5, even),   Results5), 
  maplist(length, [Results1, Results2, Results3, Results4, Results5], Lengths),
  foldl(mult, Lengths, 1, Mul),
  close(Stream).

% part1("simple.txt", Simple1).
% part1("input.txt", Part1).
% part2("simple.txt", Lengths, Part2).
% part2("input.txt", Lengths, Part2).
