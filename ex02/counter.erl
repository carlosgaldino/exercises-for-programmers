-module(counter).

-export([start/0]).

start() ->
    Input = re:replace(io:get_line("What is the input string? "), "(^\\s+)|(\\s+$)", "", [global,{return,list}]),
    io:fwrite("~s has ~p characters.~n", [Input, length(Input)]).
