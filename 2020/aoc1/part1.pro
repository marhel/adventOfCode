read_numbers_from_file(File, List):-
  open(File, read, Stream),
  read_numbers(Stream, List),
  close(Stream).

read_numbers(Stream,[]) :-
  at_end_of_stream(Stream).

read_numbers(Stream,[Num|NumList]) :-
  read_line_to_codes(Stream, Line),
  atom_codes(Atom, Line),
  atom_number(Atom, Num),
  read_numbers(Stream, NumList), !.

pick(_, []).
pick([X | T], [X | Comb]) :- pick(T, Comb).
pick([_ | T], [X | Comb]) :- pick(T, [X | Comb]).

mult(X, Y, Prod) :- Prod is X * Y.

solve(Items, Choosen, Mul) :-
  pick(Items, Choosen),
  foldl(plus, Choosen, 0, 2020),
  foldl(mult, Choosen, 1, Mul).

% read_numbers_from_file("input.txt", Nums), solve(Nums, [_, _], Part1), solve(Nums, [_, _, _], Part2).
