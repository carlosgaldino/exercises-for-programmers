-module(filter).
-export([start/0]).

start() ->
    Input = re:replace(io:get_line("Enter a list of numbers, separated by spaces: "), "(\\s+)", "", [global,{return,list}]),
    Res = lists:filtermap(fun(X) ->
                               case string:to_integer([X]) of
                                   {Y, _} when is_integer(Y) ->
                                       case Y rem 2 of
                                           0 -> {true, Y};
                                           _ -> false
                                       end;
                                   {error, Reason} ->
                                       io:format("~p~n", [Reason])
                               end
                       end,
                       Input),
    io:format("The even numbers are ~s.~n", [lists:join(" ", [integer_to_list(X) || X <- Res])]).