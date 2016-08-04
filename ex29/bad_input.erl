-module(bad_input).
-export([start/0]).

start() ->
    case io:fread("What is the rate of return? ", "~d") of
                 {ok, [X]} when X > 0 ->
                     Value = 72 / X,
                     io:format("It will take ~p years to double your initial investment.~n", [Value]);
                 _ ->
                     io:format("Sorry. That's not a valid input.~n"),
                     start()
    end.
