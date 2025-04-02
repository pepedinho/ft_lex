%%
[0-9A-Z]+ printf("NUMBER: %s\n", yytext);
"+" |
"-" |
"*" |
"/" printf("OPERATOR: %s\n", yytext);
"(" printf("OPEN PARENTHESIS\n");
")" printf("CLOSED PARENTHESIS\n");
"\"" printf("QUOTES");
oui printf("ok merci\n");
"\n" printf("NEWLINE\n");
. printf("Invalid character: %c\n", *yytext);
