-module(hello).

-export([start/0]).

start() ->
    Name = re:replace(io:get_line("What is your name? "), "(^\\s+)|(\\s+$)", "", [global,{return,list}]),
    io:fwrite("Hello, ~s, nice to meet you!~n", [Name]).
